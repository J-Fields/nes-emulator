use crate::cpu::Cpu;
use crate::instruction::Instruction;
use crate::rom::Rom;

pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn new() -> Nes {
        Nes { cpu: Cpu::new() }
    }

    pub fn execute(&mut self, rom: Rom) {
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
