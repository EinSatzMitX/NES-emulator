mod bus;
use bus::BUS;
mod cpu;
use cpu::CPU;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    /* Don't mess with this if you don't know what you're doing */

    // Create BUS and wrap it in Rc<RefCell>
    let bus = Rc::new(RefCell::new(BUS::new()));
    
    // Create CPU and pass it a reference to the BUS
    let cpu = Rc::new(RefCell::new(CPU::new(Rc::clone(&bus))));
    
    // Link the CPU to the BUS (Weak reference)
    bus.borrow_mut().link_cpu(Rc::downgrade(&cpu));
}
