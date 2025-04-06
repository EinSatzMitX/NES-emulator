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
    // let pbus = BUS::new();
    // let pcpu = CPU::new();
    // let pppu = PPU::new();
    // pcpu.borrow_mut().connect_bus(&Rc::new(pbus.clone()));
    //
    // pbus.borrow_mut().cpu = Some(pcpu); 
    // pbus.borrow_mut().ppu = Some(pppu); 
    //
    // let cartridge: Rc<RefCell<Cartridge>> = Cartridge::new("nestest.nes");
    //
    // let cartridge_dyn: Rc<RefCell<dyn ICartridge>> = cartridge;
    //
    // // Wrap the trait object in another Rc:
    // let cartridge_double: Rc<Rc<RefCell<dyn ICartridge>>> = Rc::new(cartridge_dyn);
    //
    // pbus.borrow_mut().insert_cartridge(&cartridge_double);   
    //
    // loop{
    //     // pbus.borrow_mut().clock(); 
    //
    //     // Phase 1: Borrow the bus mutably for state updates.
    //     let (cpu_clone, ppu_clone) = {
    //         let mut bus = pbus.borrow_mut();
    //         bus.n_sys_clockcounter += 1;
    //         let cpu_clone = bus.cpu.clone();
    //     let ppu_clone = bus.ppu.clone();
    //     (cpu_clone, ppu_clone)
    //   }; // The mutable borrow on `pbus` is dropped here.
    //
    //     // Phase 2: Call the clock functions without holding a borrow on `pbus`.
    //     if let Some(cpu) = cpu_clone {
    //         cpu.borrow_mut().clock();
    //         println!("cpu clocked!");
    //     }
    //     if let Some(ppu) = ppu_clone {
    //         ppu.borrow_mut().clock(); 
    //         println!("ppu clocked!");
    //     }
    // }

    // Initialize bus, CPU, and PPU.
    let pbus = BUS::new();
    let pcpu = CPU::new();
    let pppu = PPU::new();
    pcpu.borrow_mut().connect_bus(&Rc::new(pbus.clone()));

    pbus.borrow_mut().cpu = Some(pcpu); 
    pbus.borrow_mut().ppu = Some(pppu); 

    let cartridge: Rc<RefCell<Cartridge>> = Cartridge::new("nestest.nes");
    let cartridge_dyn: Rc<RefCell<dyn ICartridge>> = cartridge;
    let cartridge_double: Rc<Rc<RefCell<dyn ICartridge>>> = Rc::new(cartridge_dyn);
    pbus.borrow_mut().insert_cartridge(&cartridge_double);   

    // A separate counter to track PPU clocks.
    let mut clock_counter = 0;

    loop {
        {
            // Begin a mutable borrow block on the bus.
            let mut bus = pbus.borrow_mut();
            // Optionally update bus internal clock counter.
            bus.n_sys_clockcounter += 1;
            
            // Clock the PPU.
            if let Some(ppu) = bus.ppu.clone() {
                ppu.borrow_mut().clock();
                println!("ppu clocked!");
            }
        } // Mutable borrow on `pbus` ends here.

        // Increment the counter after the PPU has clocked.
        clock_counter += 1;

        // Every third PPU clock, clock the CPU.
        if clock_counter % 3 == 0 {
            // Obtain a new borrow for the CPU clock call.
            if let Some(cpu) = pbus.borrow().cpu.clone() {
                // Pass the bus reference if your CPU::clock() requires it.
                cpu.borrow_mut().clock();
                println!("cpu clocked!");
            }
        }
    }
}
