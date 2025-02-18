use std::{cell::RefCell, rc::{Rc, Weak}};
use crate::bus::BUS;


pub struct Mapper{
    pub prg_banks: u8,
    pub chr_banks: u8,

    pub bus: Weak<RefCell<BUS>>,
}

pub trait IMapper{
    fn new(prg_banks: u8, chr_banks: u8) -> Rc<RefCell<Self>>
    where 
        Self: Sized;

    fn cpu_map_read(&mut self, addr: u16, mapped_addr: u32) -> bool;
    fn cpu_map_write(&mut self, addr: u16, mapped_addr: u32) -> bool;
    fn ppu_map_read(&mut self, addr: u16, mapped_addr: u32) -> bool;
    fn ppu_map_write(&mut self, addr: u16, mapped_addr: u32) -> bool;

}

impl IMapper for Mapper{
    fn new(prg_banks: u8, chr_banks: u8) -> Rc<RefCell<Self>>{
        Rc::new(RefCell::new(Mapper{
            bus: Weak::new(),
            prg_banks,
            chr_banks,
        }))
    }


    fn cpu_map_read(&mut self, addr: u16, mut mapped_addr: u32) -> bool{todo!()}
    fn cpu_map_write(&mut self, addr: u16, mut mapped_addr: u32) -> bool{todo!()}
    fn ppu_map_read(&mut self, addr: u16, mut mapped_addr: u32) -> bool{todo!()}
    fn ppu_map_write(&mut self, addr: u16, mut mapped_addr: u32) -> bool{todo!()}

}
