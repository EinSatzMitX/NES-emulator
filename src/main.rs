mod bus;
use bus::BUS;
mod cpu;
use cpu::CPU;
mod ppu;
use ppu::PPU;
mod cartridge;
use cartridge::Cartridge;
mod mapper;
mod mapper000;

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use crate::cpu::ICPU;
use crate::cpu::Instruction;

use crate::ppu::IPPU;
use crate::cartridge::ICartridge;
use crate::mapper000::Mapper000;
use crate::mapper::IMapper;

fn main() {
    let pbus = BUS::new();
    let pcpu = CPU::new();
    let pppu = PPU::new();
    pcpu.borrow_mut().connect_bus(&Rc::new(pbus.clone()));

    pbus.borrow_mut().cpu = Some(pcpu); 
    pbus.borrow_mut().ppu = Some(pppu); 

    let cartridge: Rc<RefCell<Cartridge>> = Cartridge::new("nestest.nes");

    // Upcast to a trait object. Rust’s unsizing coercion will let this work:
    let cartridge_dyn: Rc<RefCell<dyn ICartridge>> = cartridge;

    // Now wrap the trait object in another Rc:
    let cartridge_double: Rc<Rc<RefCell<dyn ICartridge>>> = Rc::new(cartridge_dyn);

    // Now call insert_cartridge with the correct type:
    pbus.borrow_mut().insert_cartridge(&cartridge_double);   

    loop{
        pbus.borrow_mut().clock(); 
    }

    //let mut bus = pbus.borrow_mut();
    //
    //bus.cpu = Rc::downgrade(&Rc::new(pcpu.clone()));
    //bus.ppu = Rc::downgrade(&Rc::new(pppu));
    //
    //let cartridge: Rc<RefCell<dyn ICartridge>> = Cartridge::new("nestest.nes");
    //let cartridge_converted: Rc<Rc<RefCell<dyn ICartridge>>> = Rc::new(cartridge.clone());
    //bus.cartridge = Rc::downgrade(&cartridge_converted);
    //
    //bus.insert_cartridge(&Rc::new(cartridge));
    //
    //bus.reset();
    //
    //loop {
    //    bus.clock();
    //}
}
