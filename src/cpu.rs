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
        memory[usize::from(self.pc - 1)]
      }
      AddressMode::Relative => {
        self.pc += 1;
        memory[usize::from(self.pc - 1)]
      }
      _ => panic!("Unimplemented: {:?}", instruction.address_mode),
    };

    match instruction.opcode {
      // --- Logical operators ---
      Opcode::AND => {
        self.a &= operand;
        self.set_zero_and_sign(self.a);
      }
      Opcode::ORA => {
        self.a |= operand;
        self.set_zero_and_sign(self.a);
      }
      Opcode::EOR => {
        self.a ^= operand;
        self.set_zero_and_sign(self.a);
      }

      // --- Math operators ---
      Opcode::DEX => {
        self.x -= 1;
        self.set_zero_and_sign(self.x);
      }
      Opcode::DEY => {
        self.y -= 1;
        self.set_zero_and_sign(self.y);
      }
      Opcode::INX => {
        self.x += 1;
        self.set_zero_and_sign(self.x);
      }
      Opcode::INY => {
        self.y += 1;
        self.set_zero_and_sign(self.y);
      }

      // --- Register manipulation ---
      Opcode::TAX => {
        self.x = self.a;
        self.set_zero_and_sign(self.x);
      }
      Opcode::TAY => {
        self.y = self.a;
        self.set_zero_and_sign(self.y);
      }
      Opcode::TXA => {
        self.a = self.x;
        self.set_zero_and_sign(self.a);
      }
      Opcode::TYA => {
        self.a = self.y;
        self.set_zero_and_sign(self.a);
      }
      Opcode::TSX => {
        self.x = self.s;
        self.set_zero_and_sign(self.x);
      }
      Opcode::TXS => {
        self.s = self.x;
      }

      // --- Stack manipulation ---
      Opcode::PHA => {
        memory[usize::from(self.s)] = self.a;
        self.s += 1;
      }

      // --- Control flow ---
      Opcode::BCS => {
        self.branch(self.p.carry, operand);
      }
      Opcode::BCC => {
        self.branch(!self.p.carry, operand);
      }
      Opcode::BEQ => {
        self.branch(self.p.zero, operand);
      }
      Opcode::BNE => {
        self.branch(!self.p.zero, operand);
      }
      Opcode::BMI => {
        self.branch(self.p.sign, operand);
      }
      Opcode::BPL => {
        self.branch(!self.p.sign, operand);
      }
      Opcode::BVS => {
        self.branch(self.p.overflow, operand);
      }
      Opcode::BVC => {
        self.branch(!self.p.overflow, operand);
      }

      // --- Misc. instructions ---
      Opcode::CLC => self.p.carry = false,
      Opcode::CLD => self.p.dec_mode = false,
      Opcode::CLI => self.p.int_disabled = false,
      Opcode::CLV => self.p.overflow = false,
      Opcode::SEC => self.p.carry = true,
      Opcode::SED => self.p.dec_mode = true,
      Opcode::SEI => self.p.int_disabled = true,
      Opcode::BRK => self.p.brk = true,
      Opcode::NOP => (),
      _ => panic!("Unimplemented: {:?}", instruction.opcode),
    }

    println!("Executed instruction: {:?}", instruction);
    println!("CPU state: {:?}", self);
  }

  fn set_zero_and_sign(&mut self, register: u8) {
    self.p.zero = register == 0;
    self.p.sign = register & 0b1000_0000 != 0;
  }

  // Changes the program counter by offset if condition == true.
  // NOTE: offset is interpreted as signed, though it is not.
  fn branch(&mut self, condition: bool, offset: u8) {
    if condition {
      if offset & 0b1000_0000 != 0 {
        // It's negative
        self.pc -= u16::from(offset & 0b0111_1111);
      } else {
        self.pc += u16::from(offset & 0b0111_1111);
      }
    }
  }
}
