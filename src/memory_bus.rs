/*
  0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
  4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
  8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
  A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)
  C000-CFFF   4KB Work RAM Bank 0 (WRAM)
  D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
  E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)
  FE00-FE9F   Sprite Attribute Table (OAM)
  FEA0-FEFF   Not Usable
  FF00-FF7F   I/O Ports
  FF80-FFFE   High RAM (HRAM)
  FFFF        Interrupt Enable Register
*/
use std::fs::File;
use std::io::Read;
const BOOT_SIZE: usize = 256;
const MEMORY_SYZE: usize = 0xFFFF;
use crate::rom::Rom;
use crate::rom::Mbc;

#[derive(Debug)]
pub struct Memory {
    boot_mode: bool,
    boot_rom: Vec<u8>,
    raw_memory: Vec<u8>,
    raw_rom: Rom,
}

impl Memory {
    pub fn new() -> Self {
        let mut mem = Memory {
            boot_mode: true,
            boot_rom: Self::load_boot("ROMS/dmg_boot.bin".to_string()),
            raw_memory: vec![0u8; MEMORY_SYZE],
            //Set the boot rom on start
            raw_rom: Rom::load("ROMS/t.gb".to_string()),
        };
        if mem.raw_rom.mbc == Mbc::RomOnly {
            println!("ROM Only mode detected! Loading into RAM");
            //write ROM buffer into RAM
            
            &mem.raw_memory[0..32768].copy_from_slice(&mem.raw_rom.buffer[0..32768]);
        }
            
        mem
    }

    pub fn read_byte(&mut self, cpu: &mut crate::Cpu) -> u8{

        // We need to know first if we are in BOOT mode or in ROM mode
        // When system writes to a concrete address (FF50h) it tries to swap from one mode to other
        
        //TODO: If we are in MBC0 mode continue here:
        match cpu.registers.pc {
            0x0000...0x7FFF => {
                if(self.boot_mode) {
                    self.read_boot_byte(cpu)
                } else {
                   
                   self.raw_rom.read_byte(cpu)
                }

            },
            _=> {
                panic!("Hey! Not expected to be here!")
            },
        }

    }

    pub fn load_boot(path: String) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut file = File::open(path).expect("Invalid Boot path");
        file.read_to_end(&mut buffer).expect("Unable to read Boot file");
        buffer
    }

    pub fn read_boot_byte(&mut self, cpu: &mut crate::Cpu) -> u8{
        
        let byte = self.boot_rom[cpu.registers.pc as usize];
        let new_pc = cpu.registers.pc.wrapping_add(1);
        cpu.registers.set("PC", new_pc);
        //println!("Readed byte, pc incremented: {}", new_pc);
        byte
    }

 
}