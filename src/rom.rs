use std::fs;

pub struct ROM {}

impl ROM {
  pub fn new(rom_path: &str) -> Result<ROM, String> {
    let rom_contents = fs::read(rom_path);

    if let Ok(rom_contents) = rom_contents {
      println!("Successfully read {}", rom_path);

      let header = &rom_contents[0..16];
      let magic_string = String::from_utf8(header[0..3].to_vec());
      if let Ok(magic_string) = magic_string {
        if magic_string != "NES" || header[3] != 0x1A {
          return Err(format!("Magic string was wrong: was {}", magic_string));
        }
      } else {
        return Err("Could not parse magic string in ROM header".to_string());
      }

      // Sizes of memory regions, in bytes
      let prg_size = u32::from(header[4]) * 16000;
      let chr_size = u32::from(header[5]) * 8000;

      println!("prg_size: {}", prg_size);
      println!("chr_size: {}", chr_size);

      return Ok(ROM {});
    } else {
      return Err(format!("Encountered an error reading {}", rom_path));
    }
  }
}
