
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::task::Wake;

use crate::ICartridge;
use crate::bus::BUS;

pub struct PPU{
    //cartridge: Weak<Rc<RefCell<dyn ICartridge>>>,
    
    /* New pointer system */
    pub bus: Weak<RefCell<BUS>>,
    pub cartridge: Weak<Rc<RefCell<dyn ICartridge>>>, 

    frame_complete: bool,

    scanline: i16,
    cycle: i16,

    /* 2KB of VRAM */
    tbl_name: [[u8; 1024]; 2],
    palette: [u8; 32],
    /* unnecessary */ 
    // tbl_pattern: [[u8; 4096]; 2],
}

pub trait IPPU {

    fn new() -> Rc<RefCell<Self>>
    where 
        Self: Sized;

    /* Functions for accessing the CPU Bus */
    fn cpu_read(&mut self, addr: u16, read_only: bool) -> u8;
    fn cpu_write(&mut self, addr: u16, data: u8);

    /* Functions for accessing the PPU Bus */
    fn ppu_read(&mut self, addr: u16, read_only: bool) -> u8;
    fn ppu_write(&mut self, addr: u16, data: u8);

    fn connect_cartridge(&mut self, cartridge: &Rc<Rc<RefCell<dyn ICartridge>>>);
    fn clock(&mut self);

}

impl IPPU for PPU{

    fn new() -> Rc<RefCell<Self>>{
                Rc::new(RefCell::new(PPU {
                    bus: Weak::new(),
                    cartridge: Weak::new(), 
                    frame_complete: false,
                    scanline: 0,
                    cycle: 0,
                    tbl_name: [[0u8; 1024]; 2],
                    palette: [0u8; 32],
        }))
    }

    fn cpu_read(&mut self, addr: u16, read_only: bool) -> u8{
        let mut data: u8 = 0x00;

        match data {
            0x0000 => /* Control */ {},
            0x0001 => /* Mask */{},
            0x0002 => /* Status */{},
            0x0003 => /* OAM Address */{},
            0x0004 => /* OAM Data */{},
            0x0005 => /* Scroll */{},
            0x0006 => /* PPU Address */{},
            0x0007 => /* PPU Data */{},
            _ => {},

        }

        return data;
    }
    fn cpu_write(&mut self, addr: u16, data: u8){
        match data {
            0x0000 => /* Control */ {},
            0x0001 => /* Mask */{},
            0x0002 => /* Status */{},
            0x0003 => /* OAM Address */{},
            0x0004 => /* OAM Data */{},
            0x0005 => /* Scroll */{},
            0x0006 => /* PPU Address */{},
            0x0007 => /* PPU Data */{},
            _ => {},

        }
    }

    fn ppu_read(&mut self, mut addr: u16, read_only: bool) -> u8{
        let mut data = 0x00;
        addr &= 0x3FFF;

        let cart = self.cartridge.upgrade().unwrap();
        
        if (*cart).borrow_mut().ppu_read(addr, data){

        }

        data
    }
    fn ppu_write(&mut self, mut addr: u16, data: u8){
        addr &= 0x3FFF;

        let cart = self.cartridge.upgrade().unwrap();
        
        //if (**cart).borrow_mut().ppu_write(addr, data){
        //
        //}

    }

    fn connect_cartridge(&mut self, cartridge: &Rc<Rc<RefCell<dyn ICartridge>>>){
        self.cartridge = Rc::downgrade(cartridge);
    }
    fn clock(&mut self){

        /* DrawPixel(cycle -1, scanline); */

        self.cycle += 1;

        if self.cycle >= 341{
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 261 {
                self.scanline = -1;
                self.frame_complete = true;
            }
        }
    }

}
