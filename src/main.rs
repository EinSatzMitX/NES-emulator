mod bus;
use bus::BUS;
mod cpu;
use cpu::CPU;
mod ppu;
use ppu::PPU;
mod cartridge;
// use cartridge;
mod mapper;
mod mapper000;

use std::rc::Rc;
use crate::cpu::ICPU;
use crate::cpu::Instruction;

use crate::ppu::IPPU;

fn main() {
    let bus = BUS::new();
    let cpu = CPU::new();
    cpu.borrow_mut().connect_bus(&Rc::new(bus));

    // let lda = Instruction::new(
    //     "LDA",
    //     2,
    //     |cpu: &mut dyn ICPU| cpu.ins_lda(), 
    //     |cpu: &mut dyn ICPU| cpu.addrmode_imm()
    // );

    // (lda.opcode_function)(&mut *cpu.borrow_mut());
    
 

    
}
