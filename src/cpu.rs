use crate::instruction::AddressMode;
use crate::instruction::Instruction;
use crate::instruction::Opcode;

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
  pub pc: u16,       // Program counter
  s: u8,             // Stack pointer
  p: StatusRegister, // Status register
  a: u8,             // Accumulator
  x: u8,             // X index
  y: u8,             // Y index
}

impl CPU {
  pub fn new() -> CPU {
    CPU {
      // pc: 0x34,
      pc: 0,
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

  pub fn step(&mut self, instruction: Instruction, memory: &mut Vec<u8>) {
    let operand = match instruction.address_mode {
      AddressMode::Immediate => {
        self.pc += 1;
        memory.get(usize::from(self.pc - 1)).unwrap()
      }
      _ => panic!("Unimplemented: {:?}", instruction.address_mode),
    };

    match instruction.opcode {
      Opcode::AND => {
        self.a &= operand;
        self.p.zero = self.a == 0;
        self.p.sign = self.a & 0b1000_0000 != 0;
      }
      Opcode::ORA => {
        self.a |= operand;
        self.p.zero = self.a == 0;
        self.p.sign = self.a & 0b1000_0000 != 0;
      }
      Opcode::EOR => {
        self.a ^= operand;
        self.p.zero = self.a == 0;
        self.p.sign = self.a & 0b1000_0000 != 0;
      }
      Opcode::BRK => self.p.brk = true,
      Opcode::CLC => self.p.carry = false,
      Opcode::CLD => self.p.dec_mode = false,
      Opcode::CLI => self.p.int_disabled = false,
      Opcode::CLV => self.p.overflow = false,
      Opcode::DEX => {
        self.x -= 1;
        self.p.zero = self.x == 0;
        self.p.sign = self.x & 0b1000_0000 != 0;
      }
      Opcode::DEY => {
        self.y -= 1;
        self.p.zero = self.y == 0;
        self.p.sign = self.y & 0b1000_0000 != 0;
      }
      Opcode::INX => {
        self.x += 1;
        self.p.zero = self.x == 0;
        self.p.sign = self.x & 0b1000_0000 != 0;
      }
      Opcode::INY => {
        self.y += 1;
        self.p.zero = self.y == 0;
        self.p.sign = self.y & 0b1000_0000 != 0;
      }
      Opcode::SEC => self.p.carry = true,
      Opcode::SED => self.p.dec_mode = true,
      Opcode::SEI => self.p.int_disabled = true,
      Opcode::NOP => (),
      _ => panic!("Unimplemented: {:?}", instruction.opcode),
    }

    println!("Executed instruction: {:?}", instruction);
    println!("CPU state: {:?}", self);
  }
}
