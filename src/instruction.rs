extern crate bit_vec;

use bit_vec::BitVec;

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Opcode {
  ADC,
  AND,
  ASL,
  BCC,
  BCS,
  BEQ,
  BIT,
  BMI,
  BNE,
  BPL,
  BRK,
  BVC,
  BVS,
  CLC,
  CLD,
  CLI,
  CLV,
  CMP,
  CPX,
  CPY,
  DEC,
  DEX,
  DEY,
  EOR,
  INC,
  INX,
  INY,
  JMP,
  JSR,
  LDA,
  LDX,
  LDY,
  LSR,
  NOP,
  ORA,
  PHA,
  PHP,
  PLA,
  PLP,
  ROL,
  ROR,
  RTI,
  RTS,
  SBC,
  SEC,
  SED,
  SEI,
  STA,
  STX,
  STY,
  TAX,
  TAY,
  TSX,
  TXA,
  TXS,
  TYA,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AddressMode {
  Implicit,
  Accumulator,
  Immediate,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
  Relative,
  Absolute,
  AbsoluteX,
  AbsoluteY,
  Indirect,
  IndexedIndirect,
  IndirectIndexed,
}

#[derive(Debug)]
pub struct Instruction {
  pub opcode: Opcode,
  pub address_mode: AddressMode,
}

impl Instruction {
  pub fn new(opcode: u8) -> Instruction {
    let bv = BitVec::from_bytes(&[opcode]);

    let opcode = match (bv.get(0).unwrap(), bv.get(1).unwrap(), bv.get(2).unwrap()) {
      (false, false, false) => {
        Opcode::ORA
      },
      (false, false, true) => {
        Opcode::AND
      },
      _ => panic!("Unknown opcode")
    };

    let address_mode = match (bv.get(3).unwrap(), bv.get(4).unwrap(), bv.get(5).unwrap()) {
      (false, true, false) => {
        AddressMode::Immediate
      },
      _ => panic!("Unknown address mode")
    };

    Instruction {
      opcode,
      address_mode,
    }
  }
}
