/*  bus.rs
*   The BUS struct is one of the most important pieces of code at the moment
*   as it's communicating with absolutely everything (CPU, PPU, APU, RAM, etc.).
*   
*   Please only modify this if you know what you are doing.
*/

use crate::cartridge;
use crate::IPPU;
use crate::ICPU;
use crate::cartridge::ICartridge;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::{Rc, Weak};


pub struct BUS {
    pub cpu_ram: [u8; 2048],
    cpu: Weak<Rc<RefCell<dyn ICPU>>>,
    ppu: Weak<Rc<RefCell<dyn IPPU>>>,
    cartridge: Weak<Rc<RefCell<dyn ICartridge>>>,

    nSysClockcounter: u32,
}

impl BUS {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(BUS { 
            cpu_ram: [0; 2048], 
            cpu: Weak::new(), 
            ppu: Weak::new(), 
            cartridge: Weak::new(),
            nSysClockcounter: 0 
        }))
    }

    /* This is actually cpu_write, I was jsut too lazy to rename it */
    pub fn write(&mut self, addr: u16, data: u8) {
        let cartridge = self.cartridge.upgrade().unwrap();

        if (**cartridge).borrow_mut().cpu_write(addr, data) == true{

        } else if addr <= 0x1FFF{
            self.cpu_ram[addr as usize] = data;
        } else if addr >= 0x2000 && addr <= 0x3FFF {
            if let Some(ppu) = self.ppu.upgrade() {
                (**ppu).borrow_mut().cpu_write(addr & 0x0007, data);
            }
        }
    }

    /* This is actually cpu_read, I was jsut too lazy to rename it */
    pub fn read(&self, addr: u16, read_only: bool) -> u8 {
        let mut data: u8 = 0x00;

        let cartridge = self.cartridge.upgrade().unwrap();

        if (**cartridge).borrow_mut().cpu_read(addr, data != 0) == true{

        } else if addr <= 0x1FFF{
            data = self.cpu_ram[(addr as usize) & 0x07FF];
        } else if addr >= 0x2000 && addr <= 0x3FFF{
            if let Some(ppu) = self.ppu.upgrade(){
                data = (**ppu).borrow_mut().cpu_read(addr & 0x000f, read_only);
            }
        }
        return data;
    }

    fn insert_cartridge(&mut self, cartridge: &Rc<Rc<RefCell<dyn ICartridge>>>){
        self.cartridge = Rc::downgrade(cartridge);
        if let Some(ppu) = self.ppu.upgrade(){
            (**ppu).borrow_mut().connect_cartridge(cartridge);
        }
    }
    fn reset(&mut self){
        if let Some(cpu) = self.cpu.upgrade(){
            (**cpu).borrow_mut().reset();
            self.nSysClockcounter = 0;
        }
    }
    fn clock(&mut self){

    }

}
