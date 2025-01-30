use crate::cpu::CPU;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct BUS {
    ram: [u8; 64 * 1024],
    cpu: Weak<RefCell<CPU>>,  // Weak reference to avoid cycles
}

impl BUS {
    pub fn new() -> Self {
        BUS {
            ram: [0; 64 * 1024],
            cpu: Weak::new(),
        }
    }

    // Link the CPU to the BUS
    pub fn link_cpu(&mut self, cpu: Weak<RefCell<CPU>>) {
        self.cpu = cpu;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }

    pub fn read(&self, addr: u16, _read_only: bool) -> u8 {
        self.ram[addr as usize]
    }

    // Example method accessing CPU
    // pub fn signal_interrupt(&self) {
    //     if let Some(cpu) = self.cpu.upgrade() {
    //         cpu.borrow_mut().handle_interrupt();
    //     }
    // }
}
