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
    /// Used for instructions which do not access memory (e.g. NOP).
    Implicit,
    /// Used for instructions which operate directly on the contents of the accumulator.
    Accumulator,
    /// Single operand: constant value to operate on.
    Immediate,
    /// Single operand: pointer to address in zero page ($0000-$00FF).
    ZeroPage,
    /// Single operand: pointer to address in zero page ($0000-$00FF). Offset in register X is added.
    ZeroPageX,
    /// Single operand: pointer to address in zero page ($0000-$00FF). Offset in register Y is added.
    ZeroPageY,
    /// Single operand: signed offset to apply to program counter if some condition is met. Used for branches.
    Relative,
    /// Two operands specifying address, least significant byte first.
    Absolute,
    /// Two operands specifying address, least significant byte first. Offset in X register X is added.
    AbsoluteX,
    /// Two operands specifying address, least significant byte first. Offset in register Y is added.
    AbsoluteY,
    /// Two operands specifying address, least significant byte first. Address to use is stored at this address.
    Indirect,
    /// Single operand: offset applied to address in register X. Address to use is stored at this address.
    IndexedIndirect,
    /// Single operand: pointer to address in zero page ($0000-$00FF).
    /// Address to use is stored at this address, after adding offset in register Y.
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
            (false, false, false) => Opcode::ORA,
            (false, false, true) => Opcode::AND,
            _ => panic!("Unknown opcode"),
        };

        let address_mode = match (bv.get(3).unwrap(), bv.get(4).unwrap(), bv.get(5).unwrap()) {
            (false, true, false) => AddressMode::Immediate,
            _ => panic!("Unknown address mode"),
        };

        Instruction {
            opcode,
            address_mode,
        }
    }
}
