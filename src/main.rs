mod nes;
mod rom;

use nes::NES;
use rom::ROM;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = args.get(1);

    if let Some(rom_path) = rom_path {
        let nes = NES::new();
        let rom = ROM::new(rom_path);

        match rom {
            Ok(rom) => {
                println!("Starting game at path:  {}", rom_path);
                nes.start_game(rom);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    } else {
        print!("Expected a ROM path argument");
    }
}
