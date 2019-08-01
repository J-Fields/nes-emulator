use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::rom::ROM;

pub struct NES {
  cpu: CPU,
}

impl NES {
  pub fn new() -> NES {
    NES { cpu: CPU::new() }
  }

  pub fn execute(&mut self, rom: ROM) {
    rom.verify();
    let mut memory: Vec<u8> = vec![0x09, 0b1111_1111, 0x29, 0b0110_1011];

    while self.cpu.pc < (memory.len() as u16) {
      let inst_raw = memory.get(self.cpu.pc as usize).unwrap();
      let instruction = Instruction::new(*inst_raw);
      self.cpu.pc += 1;
      self.cpu.step(instruction, &mut memory);
    }
  }
}
