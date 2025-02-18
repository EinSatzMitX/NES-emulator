use crate::mapper::{IMapper, Mapper};

use std::rc::Rc;
use std::cell::RefCell;


pub struct Mapper000{
    prg_banks: u8,
    chr_banks: u8,
}

impl IMapper for Mapper000{

    fn new(prg_banks: u8, chr_banks: u8) -> Rc<RefCell<Self>>{
        Rc::new(RefCell::new(Mapper000{
            prg_banks,
            chr_banks,
        }))
    }


    fn cpu_map_read(&mut self, addr: u16, mut mapped_addr: u32) -> bool{
        if addr >= 0x8000 {
            mapped_addr = (addr & if self.prg_banks as u8 > 1 { 0x7FFF } else { 0x3FFF }) as u32;
            return true;
        }

        return false;
    }
    fn cpu_map_write(&mut self, addr: u16, mut mapped_addr: u32) -> bool{
        
        if addr >= 0x8000{
            mapped_addr = (addr & if self.prg_banks as u8 > 1 { 0x7FFF } else { 0x3FFF }) as u32;
            return true;
        }

        return false;
    }
    fn ppu_map_read(&mut self, addr: u16, mut mapped_addr: u32) -> bool{
            
        if addr >= 0x8000 && addr <= 0x1FFF{
            mapped_addr = addr as u32;
            return true;
        }

        return false;
    }
    fn ppu_map_write(&mut self, addr: u16, mapped_addr: u32) -> bool{
        
        // if addr >= 0x8000 && addr <= 0x1FFF{
        //
        //     return true;
        // }

        return false;
    }

}
