use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum Mbc {
    RomOnly,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc4,
    Mbc5,
    Unknown
}
#[derive(Debug)]
pub struct Rom {
    pub buffer: Vec<u8>,
    pub mbc: Mbc,
}

impl Rom {
    pub fn load(path: String) -> Rom {
        let mut mbc = Mbc::Unknown;
        let mut buffer = Vec::new();
        let mut file = File::open(path).expect("Invalid ROM path");
        file.read_to_end(&mut buffer).expect("Unable to read ROM");
        // 32Kb or more
        if(file.metadata().unwrap().len() >= 32768) {
            match buffer[0x0147] {
                0x00 => {mbc = Mbc::RomOnly},
                0x01 ... 0x03 => {mbc = Mbc::Mbc1},
                0x05 ... 0x06 => {mbc = Mbc::Mbc2},
                0x0F ... 0x13 => {mbc = Mbc::Mbc3},
                0x15 ... 0x17 => {mbc = Mbc::Mbc4},
                0x19 ... 0x1E => {mbc = Mbc::Mbc5},
                _ => {mbc = Mbc::Unknown},
            };
        }
        println!("ROM Loaded, {} bytes, MBC Mode: {:?}", file.metadata().unwrap().len(), mbc);

        Rom {
            buffer: buffer,
            mbc: mbc
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