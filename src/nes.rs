use crate::rom::ROM;

pub struct NES {}

impl NES {
  pub fn new() -> NES {
    NES {}
  }

  pub fn start_game(&self, _rom: ROM) {}
}
