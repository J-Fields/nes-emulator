use crate::cpu::CPU;
use crate::opcode::Opcode;
use crate::rom::ROM;

pub struct NES {
  cpu: CPU,
}

impl NES {
  pub fn new() -> NES {
    NES { cpu: CPU::new() }
  }

  pub fn start_game(&mut self, _rom: ROM) {
    self.cpu.step(Opcode::NOP);
  }
}
