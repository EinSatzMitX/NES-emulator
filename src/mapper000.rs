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


    fn cpu_map_read(addr: u16, mapped_addr: u32) -> bool{todo!()}
    fn cpu_map_write(addr: u16, mapped_addr: u32) -> bool{todo!()}
    fn ppu_map_read(addr: u16, mapped_addr: u32) -> bool{todo!()}
    fn ppu_map_write(addr: u16, mapped_addr: u32) -> bool{todo!()}

}
