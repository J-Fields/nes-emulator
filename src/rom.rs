extern crate bit_vec;

use bit_vec::BitVec;
use std::fs;

const KB: usize = 1024;

pub enum Mirroring {
  Horizontal,
  Vertical,
  FourScreen,
}

pub struct ROM {
  mapper: u8,
  mirroring: Mirroring,
  persistent_memory: bool,
  trainer: bool,
  v2: bool,
  pub prg_rom: Vec<u8>,
  chr_rom: Vec<u8>,
}

impl ROM {
  pub fn new(rom_path: &str) -> Result<ROM, String> {
    let rom_contents = fs::read(rom_path);

    if let Ok(rom_contents) = rom_contents {
      println!("Successfully read {}", rom_path);

      let header = &rom_contents[..16];
      let magic_string = String::from_utf8(header[..3].to_vec());
      if let Ok(magic_string) = magic_string {
        if magic_string != "NES" || header[3] != 0x1A {
          return Err(format!("Magic string was wrong: was {}", magic_string));
        }
      } else {
        return Err("Could not parse magic string in ROM header".to_string());
      }

      // Sizes of memory regions, in bytes
      let prg_size = usize::from(header[4]) * 16 * KB;
      let chr_size = usize::from(header[5]) * 8 * KB;

      let prg_rom = rom_contents[16..16 + prg_size].to_vec();
      let chr_rom = rom_contents[16 + prg_size..16 + prg_size + chr_size].to_vec();

      let flags = BitVec::from_bytes(&header[6..10]);
      let mut mirroring = if flags.get(0).unwrap() {
        Mirroring::Vertical
      } else {
        Mirroring::Horizontal
      };
      let persistent_memory = flags.get(1).unwrap();
      let trainer = flags.get(2).unwrap();
      if flags.get(3).unwrap() {
        mirroring = Mirroring::FourScreen;
      }

      // TODO: actually parse out the mapper #
      let mapper = 0;

      let v2 = flags.get(11).unwrap() && !flags.get(10).unwrap();

      // TODO: parse VS Unisystem bit and PlayChoice-10 bit

      Ok(ROM {
        mapper,
        mirroring,
        persistent_memory,
        trainer,
        v2,
        prg_rom,
        chr_rom,
      })
    } else {
      Err(format!("Encountered an error reading {}", rom_path))
    }
  }

  // Panics if the ROM uses unsupported features
  pub fn verify(&self) {
    let mut unsupported_features = Vec::new();
    if self.trainer {
      unsupported_features.push("trainer");
    }
    if self.persistent_memory {
      unsupported_features.push("persistent_memory");
    }
    if self.mapper != 0 {
      unsupported_features.push("mapper!=0");
    }
    if self.v2 {
      unsupported_features.push("NES 2.0");
    }

    if !unsupported_features.is_empty() {
      panic!(
        "ROM uses unsupported features: {}",
        unsupported_features.join(", ")
      );
    }
  }
}
