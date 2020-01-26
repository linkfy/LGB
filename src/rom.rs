use std::fs::File;
use std::io::Read;


pub struct Rom {
    buffer: Vec<u8>,
}

impl Rom {
    pub fn load(path: String) -> Rom {
        let mut buffer = Vec::new();
        let mut file = File::open(path).expect("Invalid ROM path");
        file.read_to_end(&mut buffer).expect("Unable to read ROM");
        println!("ROM Loaded");
        Rom {
            buffer: buffer,
        }
    }

    pub fn read_byte(&mut self, cpu: &mut crate::Cpu) -> u8{
        
        let byte = self.buffer[cpu.registers.pc as usize];
        let new_pc = cpu.registers.pc.wrapping_add(1);
        cpu.registers.set("PC", new_pc);
        //println!("Readed byte, pc incremented: {}", new_pc);
        byte
    }
}