
use std::fs::File;
use std::io::{BufReader, ErrorKind, Read, Seek, SeekFrom};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::mapper::IMapper;

#[repr(C)]
#[derive(Debug, Default)]
struct Header {
    name: [u8; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [u8; 5],
}

#[derive(Debug)]
enum Mirror {
    Vertical,
    Horizontal,
    OnescreenLo,
    OnescreenHi,
}

pub struct Cartridge{
    pub prg_memory: Vec<u8>,
    pub chr_memory: Vec<u8>,

    pub mapper_id: u8,
    pub prg_banks: u8,
    pub chr_banks: u8,

    pub image_valid: bool,

    pub mirror: Mirror,

    pub mapper: Weak<Rc<RefCell<dyn IMapper>>>,

}

pub trait ICartridge{

    fn new(file_name: &str) -> Rc<RefCell<Self>>
    where 
        Self: Sized;

    /* Functions for accessing the CPU Bus */
    fn cpu_read(&mut self, addr: u16, data: u8) -> bool;
    fn cpu_write(&mut self, addr: u16, data: u8) -> bool;

    /* Functions for accessing the PPU Bus */
    fn ppu_read(&mut self, addr: u16, data: u8) -> bool;
    fn ppu_write(&mut self, addr: u16, data: u8) -> bool;

}

impl ICartridge for Cartridge{

    fn new(file_name: &str) -> Rc<RefCell<Self>>{
        
        let mut cart = Cartridge {
            prg_memory: Vec::new(),
            chr_memory: Vec::new(),

            mapper_id: 0,
            prg_banks: 0,
            chr_banks: 0,

            image_valid: false,

            mirror: Mirror::Horizontal,
            mapper: Weak::new(),
        };

        let mut b_image_valid = false;
        let file = File::open(file_name).unwrap_or_else(|e| {
            if e.kind() == ErrorKind::NotFound {
                eprintln!("Error: {} was not found", file_name);
            } else {
                eprintln!("Error opening {}: {}", file_name, e);
            }
            panic!("Could not open the file");
        });

        let mut reader = BufReader::new(file);

        let mut header = Header::default();
        
        let header_size = size_of::<Header>();
        let header_buf: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut((&mut header as *mut Header) as *mut u8, header_size)
        };

        reader.read_exact(header_buf).unwrap();

        /* If a "trainer" exists (bit 2 of mapper1 is set), skip 512 bytes. */
        if header.mapper1 & 0x04 != 0 {
            reader.seek(SeekFrom::Current(512)).unwrap();
        }
    
        // Determine Mapper ID.
        cart.mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);

        cart.mirror = if header.mapper1 & 0x01 != 0 {
            Mirror::Vertical
        } else {
            Mirror::Horizontal
        };

        let n_file_type = 1;

        cart.prg_banks = header.prg_rom_chunks;
        cart.chr_banks = header.chr_rom_chunks;
       
        if n_file_type == 0{

        }

        if n_file_type == 1 {
            let prg_size = cart.prg_banks as usize * 16384;
            cart.prg_memory.resize(prg_size as usize, 0);
            reader.read_exact(&mut cart.prg_memory).unwrap();

            let chr_size = cart.chr_banks as usize * 8192;
            cart.chr_memory.resize(chr_size as usize, 0);
            reader.read_exact(&mut cart.chr_memory).unwrap();
        }


        if n_file_type == 2{

        }


        if cart.mapper_id == 0 {
            println!("Using Mapper_000 with {} PRG banks and {} CHR banks", cart.prg_banks, cart.chr_banks);
        }

        cart.image_valid = true;
    
        Rc::new(RefCell::new(cart))
    }

    fn cpu_read(&mut self, addr: u16, mut data: u8) -> bool{
        let mut mapped_addr: u32 = 0;
        if let Some(mapper) = self.mapper.upgrade(){
            if (**mapper).borrow_mut().cpu_map_read(addr, mapped_addr){
                data = self.prg_memory[mapped_addr as usize];
                return true;
            }
        }
        return false;
    }
    fn cpu_write(&mut self, addr: u16, mut data: u8) -> bool {
        let mut mapped_addr: u32 = 0;
        if let Some(mapper) = self.mapper.upgrade(){
            if (**mapper).borrow_mut().cpu_map_write(addr, mapped_addr){
                data = self.prg_memory[mapped_addr as usize];
                return true;
            }
        }
        return false;
    }

    fn ppu_read(&mut self, addr: u16, mut data: u8) -> bool{
        let mut mapped_addr: u32 = 0;
        if let Some(mapper) = self.mapper.upgrade(){
            if (**mapper).borrow_mut().ppu_map_read(addr, mapped_addr){
                data = self.chr_memory[mapped_addr as usize];
                return true;
            }
        }
        return false;
    }
    fn ppu_write(&mut self, addr: u16, mut data: u8) -> bool {
        let mut mapped_addr: u32 = 0;
        if let Some(mapper) = self.mapper.upgrade(){
            if (**mapper).borrow_mut().cpu_map_read(addr, mapped_addr){
                data = self.chr_memory[mapped_addr as usize];
                return true;
            }
        }
        return false;
    }

}
