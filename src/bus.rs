/*  bus.rs
*   The BUS struct is one of the most important pieces of code at the moment
*   as it's communicating with absolutely everything (CPU, PPU, APU, RAM, etc.).
*   
*   Please only modify this if you know what you are doing.
*/

use crate::ICPU;

use std::cell::RefCell;
use std::rc::{Rc, Weak};


pub struct BUS {
    pub ram: [u8; 64 * 1024],
    cpu: Weak<Rc<RefCell<dyn ICPU>>>,
}

impl BUS {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(BUS { ram: [0; 64*1024], cpu: Weak::new() }))
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }

    pub fn read(&self, addr: u16, _read_only: bool) -> u8 {
        return self.ram[addr as usize];
    }

}
