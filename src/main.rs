mod cpu;
mod nes;
mod instruction;
mod rom;

use nes::Nes;
use rom::Rom;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = args.get(1);

    if let Some(rom_path) = rom_path {
        let mut nes = Nes::new();
        let rom = Rom::new(rom_path);

        match rom {
            Ok(rom) => {
                println!("Starting game at path:  {}", rom_path);
                nes.execute(rom);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    } else {
        print!("Expected a ROM path argument");
    }
}
