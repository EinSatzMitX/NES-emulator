use crate::bus::BUS;

use std::cell::RefCell;
use std::rc::Rc;

pub struct CPU {
    bus: Rc<RefCell<BUS>>,
}

impl CPU {
    pub fn new(bus: Rc<RefCell<BUS>>) -> Self {
        CPU { bus }
    }

    // Example method accessing BUS
    fn read(&self, addr: u16) -> u8 {
        self.bus.borrow().read(addr, false)
    }

    fn write(&self, addr: u16, data: u8) {
        self.bus.borrow_mut().write(addr, data);
    }
}
