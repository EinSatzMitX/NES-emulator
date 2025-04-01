/*  bus.rs
*   The BUS struct is one of the most important pieces of code at the moment
*   as it's communicating with absolutely everything (CPU, PPU, APU, RAM, etc.).
*   
*   Please only modify this if you know what you are doing.
*/

use crate::cartridge;
use crate::CPU;
use crate::PPU;
use crate::IPPU;
use crate::ICPU;
use crate::ICartridge;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::{Rc, Weak};


pub struct BUS {
    pub cpu_ram: [u8; 2048],
    //pub cpu: Weak<Rc<RefCell<dyn ICPU>>>,
    //pub ppu: Weak<Rc<RefCell<dyn IPPU>>>,
    //pub cartridge: Weak<Rc<RefCell<dyn ICartridge>>>,
    
    /* New pointer system */
    pub cpu: Option<Rc<RefCell<dyn ICPU>>>,
    pub ppu: Option<Rc<RefCell<dyn IPPU>>>,
    pub cartridge: Option<Rc<RefCell<dyn ICartridge>>>,
    
    pub n_sys_clockcounter: u32,
}

impl BUS {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(BUS { 
            cpu_ram: [0; 2048], 
            //cpu: Weak::new(), 
            //ppu: Weak::new(), 
            //cartridge: Weak::new(),
            cpu: None,
            ppu: None,
            cartridge: None,
            n_sys_clockcounter: 0 
        }))
    }

    /* This is actually cpu_write, I was jsut too lazy to rename it */
    pub fn write(&mut self, addr: u16, data: u8) {
        if let Some(cartridge) = self.cartridge.as_ref() {
            if (**cartridge).borrow_mut().cpu_write(addr, data) == false {
                return;
            }
        }

        if addr <= 0x1FFF {
            self.cpu_ram[addr as usize] = data;  
        } else if addr >= 0x2000 && addr >= 0x3FFF {
            if let Some(ppu) = self.ppu.as_ref() {
                (**ppu).borrow_mut().cpu_write(addr & 0x0007, data);
            }
        }
    }

    /* This is actually cpu_read, I was jsut too lazy to rename it */
    pub fn read(&self, addr: u16, readonly: bool) -> u8 {
        let mut data: u8 = 0x00;
        
        if let Some(cartridge) = self.cartridge.as_ref(){
            if (**cartridge).borrow_mut().cpu_read(addr, data) == false{
                // !!! TODO: look if this should actually return 0
                return 0;
            }
        }

        if addr <= 0x1FFF{
            data = self.cpu_ram[(addr as usize) & 0x07FF];
        } else if addr >= 0x2000 && addr <= 0x3FFF{
            if let Some(ppu) = self.ppu.as_ref(){
                data = (**ppu).borrow_mut().cpu_read(addr & 0x000f, readonly);
            }
        }
        return data;
    }

    pub fn insert_cartridge(&mut self, cartridge: &Rc<Rc<RefCell<dyn ICartridge>>>){
        if let Some(ppu) = self.ppu.as_ref() {
            (**ppu).borrow_mut().connect_cartridge(cartridge);
        }
    }
    pub fn reset(&mut self){
        if let Some(cpu) = self.cpu.as_ref(){
            (**cpu).borrow_mut().reset();
            self.n_sys_clockcounter = 0;
        }
    }
    pub fn clock(&mut self){

        println!("running bus.clock()!");
    
    // Clone the CPU and PPU pointers from the bus.
    let cpu_clone = self.cpu.clone();
    let ppu_clone = self.ppu.clone();
    
    // Update clock counter while still inside the mutable borrow if needed.
        self.n_sys_clockcounter += 1;
    let counter = self.n_sys_clockcounter;
        self.trigger_clock(cpu_clone, ppu_clone, counter.into()); 

    //// Drop the mutable borrow on self by ending this block.
    //// (We are no longer accessing self after this point.)
    //let _ = &mut *self;
    //
    //// Now call the clock methods on CPU and PPU without holding the bus borrow.
    //if let Some(ppu) = ppu_clone {
    //    (*ppu).borrow_mut().clock();
    //    println!("ppu clocked!");
    //}
    //
    //if self.n_sys_clockcounter % 3 == 0 {
    //    if let Some(cpu) = cpu_clone {
    //        (*cpu).borrow_mut().clock();
    //        println!("cpu clocked!");
    //    }
    //}
    }
    pub fn trigger_clock(&self, cpu_clone: Option<Rc<RefCell<dyn ICPU>>>, ppu_clone: Option<Rc<RefCell<dyn IPPU>>>, clock_counter: u64,){
        if let Some(ppu) = ppu_clone{
            (*ppu).borrow_mut().clock();
            println!("ppu clocked!");
        }
        if clock_counter % 3 == 0 {
            if let Some(cpu) = cpu_clone {
                (*cpu).borrow_mut().clock();
                println!("cpu clocked!");
            }
        }
    }

}
