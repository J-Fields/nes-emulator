use crate::opcode::Opcode;

#[derive(Debug)]
struct StatusRegister {
  carry: bool,
  zero: bool,
  int_disabled: bool,
  dec_mode: bool,
  brk: bool,
  overflow: bool,
  sign: bool,
}

#[derive(Debug)]
pub struct CPU {
  pc: u16,           // Program counter
  s: u8,             // Stack pointer
  p: StatusRegister, // Status register
  a: u8,             // Accumulator
  x: u8,             // X index
  y: u8,             // Y index
}

impl CPU {
  pub fn new() -> CPU {
    CPU {
      pc: 0x34,
      s: 0xFD,
      p: StatusRegister {
        carry: false,
        zero: false,
        int_disabled: false,
        dec_mode: false,
        brk: false,
        overflow: false,
        sign: false,
      },
      a: 0,
      x: 0,
      y: 0,
    }
  }

  pub fn step(&mut self, opcode: Opcode) {
    match opcode {
      Opcode::BRK => self.p.brk = true,
      Opcode::CLC => self.p.carry = false,
      Opcode::CLD => self.p.dec_mode = false,
      Opcode::CLI => self.p.int_disabled = false,
      Opcode::CLV => self.p.overflow = false,
      Opcode::SEC => self.p.carry = true,
      Opcode::SED => self.p.dec_mode = true,
      Opcode::SEI => self.p.int_disabled = true,
      Opcode::NOP => (),
      _ => panic!("Unimplemented: {:?}", opcode),
    }
    println!("CPU state: {:?}", self);
  }
}
