use std::{cell::RefCell, rc::Rc};

pub struct Mapper{
    pub prg_banks: u8,
    pub chr_banks: u8,
}

pub trait IMapper{
    fn new(prg_banks: u8, chr_banks: u8) -> Rc<RefCell<Self>>
    where 
        Self: Sized;

    fn cpu_map_read(addr: u16, mapped_addr: u32) -> bool;
    fn cpu_map_write(addr: u16, mapped_addr: u32) -> bool;
    fn ppu_map_read(addr: u16, mapped_addr: u32) -> bool;
    fn ppu_map_write(addr: u16, mapped_addr: u32) -> bool;

}

impl IMapper for Mapper{
    fn new(prg_banks: u8, chr_banks: u8) -> Rc<RefCell<Self>>{
        Rc::new(RefCell::new(Mapper{
            prg_banks,
            chr_banks,
        }))
    }


    fn cpu_map_read(addr: u16, mapped_addr: u32) -> bool{todo!()}
    fn cpu_map_write(addr: u16, mapped_addr: u32) -> bool{todo!()}
    fn ppu_map_read(addr: u16, mapped_addr: u32) -> bool{todo!()}
    fn ppu_map_write(addr: u16, mapped_addr: u32) -> bool{todo!()}

}
