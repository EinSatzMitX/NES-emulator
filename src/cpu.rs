/*  cpu.rs
*   The heart of the NES and any computer in general: The CPU.
*
*   The NES runs an 2A03 CPU, which is a slightly modified version of the 
*   famous 6502, developped by MOS Technology and released in 1975.
*
*   If you want to know more about the 6502 processor, I would highly recommend to watch this
*   youtube video: https://www.youtube.com/watch?v=lP2ZBp9O0mk
*
*   Back to the Programming! In this file you will not only find the CPU struct, together with the
*   ICPU trait, which holds some function declarations, you will also find the Instruction struct,
*   which points to the corresponding method inside CPU.
*   Currently there are no implementations for the different CPU instructions, but if I or whoever
*   is reading this code feels like they have too much time, they are welcome to create methods for
*   all instructions and a table filled with Instruction, which are all pointing to their methods.
*
*/


use crate::bus::BUS;

use std::cell::RefCell;
use std::io::Seek;
use std::rc::{Rc, Weak};



type OpcodeFunction = Box<dyn Fn(&mut dyn ICPU)>;
type AddrmodeFunction = Box<dyn Fn(&mut dyn ICPU)>;

pub struct Instruction{
    pub name: String,
    pub cycles: u8,
    pub opcode_function: OpcodeFunction,
    pub addrmode_function: AddrmodeFunction,
}

impl Instruction{
    pub fn new(
        name: impl Into<String>,
        cycles: u8,
        opcode_fn: fn(&mut dyn ICPU),
        addrmode_fn: fn(&mut dyn ICPU) -> u8,
    ) -> Self {
        Self {
            name: name.into(),
            cycles,
            opcode_function: Box::new(move |cpu: &mut dyn ICPU| opcode_fn(cpu)),
            addrmode_function: Box::new(move |cpu: &mut dyn ICPU| {addrmode_fn(cpu);}),
        }
    }
    pub fn new_empty() -> Self {
        Self {
            name: "XXX".to_string(),
            cycles: 2,
            opcode_function: Box::new(|cpu: &mut dyn ICPU| {
                cpu.ins_xxx();
            }),
            addrmode_function: Box::new(|cpu: &mut dyn ICPU| {
                cpu.addrmode_imp();
            }),
        }
    }
}


pub struct CPU {
    /* All members of the CPU struct will be public for now, just to make debugging/testing a bit easier*/
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub status_flags: u8, /*
    7  bit  0
    ---- ----
    NV1B DIZC
    |||| ||||
    |||| |||+- Carry
    |||| ||+-- Zero
    |||| |+--- Interrupt Disable
    |||| +---- Decimal (Only on the 6502, not the 2A03)
    |||+------ Break Flag 
    ||+------- (No CPU effect; always pushed as 1)
    |+-------- Overflow
    +--------- Negative
    */

    pub bus: Weak<Rc<RefCell<BUS>>>,

    /* Some helper members */
    pub last_fetched: u8,
    pub absolute_addr: u16,
    pub relative_addr: u16,
    pub opcode: u8,
    pub cycles: u8,
    pub lookup: Vec<Instruction>,
}

#[allow(dead_code)]
pub trait ICPU {
    fn new() -> Rc<RefCell<Self>>
    where 
        Self: Sized;
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
    fn connect_bus(&mut self, bus: &Rc<Rc<RefCell<BUS>>>);
    fn set_flag(&mut self, status_flag: u8);

    fn fetch(&mut self) -> u8;
    fn clock(&mut self);
    fn reset(&mut self);
    fn irq(&mut self);
    fn nmi(&mut self);

    /* Opcode functions */
    fn ins_adc(&mut self);
    fn ins_and(&mut self);
    fn ins_asl(&mut self);
    fn ins_bcc(&mut self);
    fn ins_bcs(&mut self);
    fn ins_beq(&mut self);
    fn ins_bit(&mut self);
    fn ins_bmi(&mut self);
    fn ins_bne(&mut self);
    fn ins_bpl(&mut self);
    fn ins_brk(&mut self);
    fn ins_bvc(&mut self);
    fn ins_bvs(&mut self);
    fn ins_clc(&mut self);
    fn ins_cld(&mut self);
    fn ins_cli(&mut self);
    fn ins_clv(&mut self);
    fn ins_cmp(&mut self);
    fn ins_cpx(&mut self);
    fn ins_cpy(&mut self);
    fn ins_dec(&mut self);
    fn ins_dex(&mut self);
    fn ins_dey(&mut self);
    fn ins_eor(&mut self);
    fn ins_inc(&mut self);
    fn ins_inx(&mut self);
    fn ins_iny(&mut self);
    fn ins_jmp(&mut self);
    fn ins_jsr(&mut self);
    fn ins_lda(&mut self);
    fn ins_ldx(&mut self);
    fn ins_ldy(&mut self);
    fn ins_lsr(&mut self);
    fn ins_nop(&mut self);
    fn ins_ora(&mut self);
    fn ins_pha(&mut self);
    fn ins_php(&mut self);
    fn ins_pla(&mut self);
    fn ins_plp(&mut self);
    fn ins_rol(&mut self);
    fn ins_ror(&mut self);
    fn ins_rti(&mut self);
    fn ins_rts(&mut self);
    fn ins_sbc(&mut self);
    fn ins_sec(&mut self);
    fn ins_sed(&mut self);
    fn ins_sei(&mut self);
    fn ins_sta(&mut self);
    fn ins_stx(&mut self);
    fn ins_sty(&mut self);
    fn ins_tax(&mut self);
    fn ins_tay(&mut self);
    fn ins_tsx(&mut self);
    fn ins_txa(&mut self);
    fn ins_txs(&mut self);
    fn ins_tya(&mut self);



    // Illegal opcode
    fn ins_xxx(&mut self);

    /* Addressing modes */
    fn addrmode_imp(&mut self) -> u8;
    fn addrmode_zp0(&mut self) -> u8;
    fn addrmode_zpy(&mut self) -> u8;
    fn addrmode_abs(&mut self) -> u8;
    fn addrmode_aby(&mut self) -> u8;
    fn addrmode_izx(&mut self) -> u8;
    fn addrmode_imm(&mut self) -> u8;
    fn addrmode_zpx(&mut self) -> u8;
    fn addrmode_rel(&mut self) -> u8;
    fn addrmode_abx(&mut self) -> u8;
    fn addrmode_ind(&mut self) -> u8;
    fn addrmode_izy(&mut self) -> u8;

    // Illegal addressing mode
    fn addrmode_xxx(&mut self) -> u8;
}

impl ICPU for CPU {
    fn new() -> Rc<RefCell<Self>> {
        let mut lookup: Vec<Instruction> = (0..256).map(|_| Instruction::new_empty()).collect();
        /* Setting up The lookup table... I hate my life */
        /* Using This as my template: https://github.com/OneLoneCoder/olcNES/blob/master/Part%232%20-%20CPU/olc6502.cpp */
        /* 0x0n Opcodes */
        lookup[0x00] = Instruction::new("BRK", 2, |cpu: &mut dyn ICPU| cpu.ins_brk(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x01] = Instruction::new("ORA", 6, |cpu: &mut dyn ICPU| cpu.ins_ora(), |cpu: &mut dyn ICPU| cpu.addrmode_izx());
        lookup[0x02] = Instruction::new("???", 2, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x03] = Instruction::new("???", 8, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x04] = Instruction::new("???", 3, |cpu: &mut dyn ICPU| cpu.ins_nop(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x05] = Instruction::new("ORA", 3, |cpu: &mut dyn ICPU| cpu.ins_ora(), |cpu: &mut dyn ICPU| cpu.addrmode_zp0());
        lookup[0x06] = Instruction::new("ASL", 5, |cpu: &mut dyn ICPU| cpu.ins_asl(), |cpu: &mut dyn ICPU| cpu.addrmode_zp0());
        lookup[0x07] = Instruction::new("???", 5, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x08] = Instruction::new("PHP", 3, |cpu: &mut dyn ICPU| cpu.ins_php(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x09] = Instruction::new("ORA", 2, |cpu: &mut dyn ICPU| cpu.ins_ora(), |cpu: &mut dyn ICPU| cpu.addrmode_imm());
        lookup[0x0A] = Instruction::new("ASL", 2, |cpu: &mut dyn ICPU| cpu.ins_asl(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x0B] = Instruction::new("???", 2, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x0C] = Instruction::new("???", 4, |cpu: &mut dyn ICPU| cpu.ins_nop(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x0D] = Instruction::new("ORA", 4, |cpu: &mut dyn ICPU| cpu.ins_ora(), |cpu: &mut dyn ICPU| cpu.addrmode_abs());
        lookup[0x0E] = Instruction::new("ASL", 6, |cpu: &mut dyn ICPU| cpu.ins_asl(), |cpu: &mut dyn ICPU| cpu.addrmode_abs());
        lookup[0x0F] = Instruction::new("???", 6, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());

        /* 0x1n Opcodes */
        lookup[0x10] = Instruction::new("BPL", 2, |cpu: &mut dyn ICPU| cpu.ins_bpl(), |cpu: &mut dyn ICPU| cpu.addrmode_rel());
        lookup[0x11] = Instruction::new("ORA", 5, |cpu: &mut dyn ICPU| cpu.ins_ora(), |cpu: &mut dyn ICPU| cpu.addrmode_izy());
        lookup[0x12] = Instruction::new("???", 2, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x13] = Instruction::new("???", 8, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x14] = Instruction::new("???", 4, |cpu: &mut dyn ICPU| cpu.ins_nop(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x15] = Instruction::new("ORA", 4, |cpu: &mut dyn ICPU| cpu.ins_ora(), |cpu: &mut dyn ICPU| cpu.addrmode_zpx());
        lookup[0x16] = Instruction::new("ASL", 6, |cpu: &mut dyn ICPU| cpu.ins_asl(), |cpu: &mut dyn ICPU| cpu.addrmode_zpx());
        lookup[0x17] = Instruction::new("???", 6, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x18] = Instruction::new("CLC", 2, |cpu: &mut dyn ICPU| cpu.ins_clc(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x19] = Instruction::new("ORA", 4, |cpu: &mut dyn ICPU| cpu.ins_ora(), |cpu: &mut dyn ICPU| cpu.addrmode_aby());
        lookup[0x1A] = Instruction::new("???", 2, |cpu: &mut dyn ICPU| cpu.ins_nop(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x1B] = Instruction::new("???", 7, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x1C] = Instruction::new("???", 4, |cpu: &mut dyn ICPU| cpu.ins_nop(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x1D] = Instruction::new("ORA", 4, |cpu: &mut dyn ICPU| cpu.ins_ora(), |cpu: &mut dyn ICPU| cpu.addrmode_abx());
        lookup[0x1E] = Instruction::new("ASL", 7, |cpu: &mut dyn ICPU| cpu.ins_asl(), |cpu: &mut dyn ICPU| cpu.addrmode_abx());
        lookup[0x1F] = Instruction::new("???", 7, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());

        
        /* 0x2n Opcodes */
        lookup[0x20] = Instruction::new("JSR", 6, |cpu: &mut dyn ICPU| cpu.ins_jsr(), |cpu: &mut dyn ICPU| cpu.addrmode_abs());
        lookup[0x21] = Instruction::new("AND", 6, |cpu: &mut dyn ICPU| cpu.ins_and(), |cpu: &mut dyn ICPU| cpu.addrmode_izx());
        lookup[0x22] = Instruction::new("???", 2, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x23] = Instruction::new("???", 8, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x24] = Instruction::new("BIT", 3, |cpu: &mut dyn ICPU| cpu.ins_bit(), |cpu: &mut dyn ICPU| cpu.addrmode_zp0());
        lookup[0x25] = Instruction::new("AND", 3, |cpu: &mut dyn ICPU| cpu.ins_and(), |cpu: &mut dyn ICPU| cpu.addrmode_zp0());
        lookup[0x26] = Instruction::new("ROL", 5, |cpu: &mut dyn ICPU| cpu.ins_rol(), |cpu: &mut dyn ICPU| cpu.addrmode_zp0());
        lookup[0x27] = Instruction::new("???", 5, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x28] = Instruction::new("PLP", 4, |cpu: &mut dyn ICPU| cpu.ins_plp(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x29] = Instruction::new("AND", 2, |cpu: &mut dyn ICPU| cpu.ins_and(), |cpu: &mut dyn ICPU| cpu.addrmode_imm());
        lookup[0x2A] = Instruction::new("ROL", 2, |cpu: &mut dyn ICPU| cpu.ins_rol(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x2B] = Instruction::new("???", 2, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x2C] = Instruction::new("BIT", 4, |cpu: &mut dyn ICPU| cpu.ins_bit(), |cpu: &mut dyn ICPU| cpu.addrmode_abs());
        lookup[0x2D] = Instruction::new("AND", 4, |cpu: &mut dyn ICPU| cpu.ins_and(), |cpu: &mut dyn ICPU| cpu.addrmode_abs());
        lookup[0x2E] = Instruction::new("ROL", 6, |cpu: &mut dyn ICPU| cpu.ins_rol(), |cpu: &mut dyn ICPU| cpu.addrmode_abs());
        lookup[0x2F] = Instruction::new("???", 6, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());

        /* 0x3n Opcodes */
        lookup[0x30] = Instruction::new("BMI", 2, |cpu: &mut dyn ICPU| cpu.ins_bmi(), |cpu: &mut dyn ICPU| cpu.addrmode_rel());
        lookup[0x31] = Instruction::new("AND", 5, |cpu: &mut dyn ICPU| cpu.ins_and(), |cpu: &mut dyn ICPU| cpu.addrmode_izy());
        lookup[0x32] = Instruction::new("???", 2, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x33] = Instruction::new("???", 8, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x34] = Instruction::new("???", 4, |cpu: &mut dyn ICPU| cpu.ins_nop(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x35] = Instruction::new("AND", 4, |cpu: &mut dyn ICPU| cpu.ins_and(), |cpu: &mut dyn ICPU| cpu.addrmode_zpx());
        lookup[0x36] = Instruction::new("ROL", 6, |cpu: &mut dyn ICPU| cpu.ins_rol(), |cpu: &mut dyn ICPU| cpu.addrmode_zpx());
        lookup[0x37] = Instruction::new("???", 6, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x38] = Instruction::new("SEC", 2, |cpu: &mut dyn ICPU| cpu.ins_sec(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x39] = Instruction::new("AND", 4, |cpu: &mut dyn ICPU| cpu.ins_and(), |cpu: &mut dyn ICPU| cpu.addrmode_aby());
        lookup[0x3A] = Instruction::new("???", 2, |cpu: &mut dyn ICPU| cpu.ins_nop(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x3B] = Instruction::new("???", 7, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x3C] = Instruction::new("NOP", 4, |cpu: &mut dyn ICPU| cpu.ins_nop(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());
        lookup[0x3D] = Instruction::new("AND", 4, |cpu: &mut dyn ICPU| cpu.ins_and(), |cpu: &mut dyn ICPU| cpu.addrmode_abx());
        lookup[0x3E] = Instruction::new("ROL", 7, |cpu: &mut dyn ICPU| cpu.ins_rol(), |cpu: &mut dyn ICPU| cpu.addrmode_abx());
        lookup[0x3F] = Instruction::new("???", 7, |cpu: &mut dyn ICPU| cpu.ins_xxx(), |cpu: &mut dyn ICPU| cpu.addrmode_imp());


        Rc::new(RefCell::new(CPU {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            status_flags: 0,
            bus: Weak::new(),
            last_fetched: 0,
            absolute_addr: 0,
            relative_addr: 0,
            opcode: 0,
            cycles: 0,
            lookup,
        }))
    }

    fn read(&self, addr: u16) -> u8 {
        if let Some(bus) = self.bus.upgrade() {
            return bus.borrow().read(addr, true);
        }
        0 // Default return if bus is not available
    }

    fn write(&mut self, addr: u16, data: u8) {
        if let Some(bus) = self.bus.upgrade() {
            bus.borrow_mut().write(addr, data);
        }
    }

    fn connect_bus(&mut self, bus: &Rc<Rc<RefCell<BUS>>>) { 
        self.bus = Rc::downgrade(bus); 
    }

    fn set_flag(&mut self, status_flag: u8){
        self.status_flags |= status_flag; 
    }
    fn fetch(&mut self) -> u8{
        todo!()
    }
    fn clock(&mut self){
        if self.cycles == 0 {
            self.opcode = self.read(self.pc);
            self.pc += 1;

        }
    }
    fn reset(&mut self){
        todo!()
    }
    fn irq(&mut self){
        todo!()
    }
    fn nmi(&mut self){
        todo!()
    }

    /* Opcodes */
    fn ins_adc(&mut self){todo!()}
    fn ins_and(&mut self){todo!()}
    fn ins_asl(&mut self){todo!()}
    fn ins_bcc(&mut self){todo!()}
    fn ins_bcs(&mut self){todo!()}
    fn ins_beq(&mut self){todo!()}
    fn ins_bit(&mut self){todo!()}
    fn ins_bmi(&mut self){todo!()}
    fn ins_bne(&mut self){todo!()}
    fn ins_bpl(&mut self){todo!()}
    fn ins_brk(&mut self){todo!()}
    fn ins_bvc(&mut self){todo!()}
    fn ins_bvs(&mut self){todo!()}
    fn ins_clc(&mut self){todo!()}
    fn ins_cld(&mut self){todo!()}
    fn ins_cli(&mut self){todo!()}
    fn ins_clv(&mut self){todo!()}
    fn ins_cmp(&mut self){todo!()}
    fn ins_cpx(&mut self){todo!()}
    fn ins_cpy(&mut self){todo!()}
    fn ins_dec(&mut self){todo!()}
    fn ins_dex(&mut self){todo!()}
    fn ins_dey(&mut self){todo!()}
    fn ins_eor(&mut self){todo!()}
    fn ins_inc(&mut self){todo!()}
    fn ins_inx(&mut self){todo!()}
    fn ins_iny(&mut self){todo!()}
    fn ins_jmp(&mut self){todo!()}
    fn ins_jsr(&mut self){todo!()}
    fn ins_lda(&mut self){todo!()}
    fn ins_ldx(&mut self){todo!()}
    fn ins_ldy(&mut self){todo!()}
    fn ins_lsr(&mut self){todo!()}
    fn ins_nop(&mut self){todo!()}
    fn ins_ora(&mut self){todo!()}
    fn ins_pha(&mut self){todo!()}
    fn ins_php(&mut self){todo!()}
    fn ins_pla(&mut self){todo!()}
    fn ins_plp(&mut self){todo!()}
    fn ins_rol(&mut self){todo!()}
    fn ins_ror(&mut self){todo!()}
    fn ins_rti(&mut self){todo!()}
    fn ins_rts(&mut self){todo!()}
    fn ins_sbc(&mut self){todo!()}
    fn ins_sec(&mut self){todo!()}
    fn ins_sed(&mut self){todo!()}
    fn ins_sei(&mut self){todo!()}
    fn ins_sta(&mut self){todo!()}
    fn ins_stx(&mut self){todo!()}
    fn ins_sty(&mut self){todo!()}
    fn ins_tax(&mut self){todo!()}
    fn ins_tay(&mut self){todo!()}
    fn ins_tsx(&mut self){todo!()}
    fn ins_txa(&mut self){todo!()}
    fn ins_txs(&mut self){todo!()}
    fn ins_tya(&mut self){todo!()}

    // Illegal opcode
    fn ins_xxx(&mut self){
        println!("Illegal opcode detected!");
    }

    /* Addressing modes */
    fn addrmode_imp(&mut self) -> u8 {
        todo!()
    }
    fn addrmode_zp0(&mut self) -> u8{
        todo!()
    }
    fn addrmode_zpy(&mut self) -> u8{
        todo!()
    }
    fn addrmode_abs(&mut self) -> u8 {
        todo!()
    }
    fn addrmode_aby(&mut self) -> u8 {
        todo!()
    }
    fn addrmode_izx(&mut self) -> u8 {
        todo!()
    }
    fn addrmode_imm(&mut self) -> u8 {
        todo!()
    }
    fn addrmode_zpx(&mut self) -> u8 {
        todo!()
    }
    fn addrmode_rel(&mut self) -> u8 {
        todo!()
    }
    fn addrmode_abx(&mut self) -> u8 {
        todo!()
    }
    fn addrmode_ind(&mut self) -> u8 {
        todo!()
    }
    fn addrmode_izy(&mut self) -> u8{
        todo!()
    }
    
    // Illegal addrmode
    fn addrmode_xxx(&mut self) -> u8 {
        println!("Illegal Addressing mode detected!");
        0
    }
}

