#[derive(Debug)]
pub struct Registers {
    register: Vec<u8>,
    pub sp: u16,
    pub pc: u16,
}
impl Registers {

    pub fn new() -> Self {
        // This is not the best option, we are crafting a vector of 77+1 elements, and we only want to use 8
        // But i think it is better to read the code easily
        // register['L' as usize] = register[77];
        Registers {
            register: vec![0; 'L' as usize +1],
            sp: 0x0000,
            pc: 0x0000,
        }
    }

    pub fn set(&mut self, reg: &str, val: u16) {
        
        match reg.len() {
            1 => {
                //If it is 1 char, set it
                let ch = reg.chars().next().unwrap().to_ascii_uppercase();
                self.register[ch as usize] = val as u8;
            },
            2 => {
                if reg.to_ascii_uppercase() == "SP" {
                    self.sp = val;
                } else if reg.to_ascii_uppercase() == "PC" {
                    self.pc = val;
                } else {
                    let mut chars = reg.chars();
                    let ch1 = chars.next().unwrap().to_ascii_uppercase();
                    let ch2 = chars.next().unwrap().to_ascii_uppercase();
    
                    
                    let val1 = val >> 8;
                    let val2 = (val & 0x00FF) as u8;
                    
                    self.register[ch1 as usize] = val1 as u8;
                    self.register[ch2 as usize] = val2 as u8;
                }

            },
            _ => panic!("Error setting reg values")
        }
        
        
    }

    pub fn get(self, reg: &str) -> u8 {
        let ch = reg.chars().next().unwrap().to_ascii_uppercase();
        self.register[ch as usize]
    }

    pub fn print(&self) {
        println!("
╔═══════╦══════════╗
║ A: {:0>2X} ║ {:0>8b} ║
║ B: {:0>2X} ║ {:0>8b} ║
║ C: {:0>2X} ║ {:0>8b} ║
║ D: {:0>2X} ║ {:0>8b} ║
║ E: {:0>2X} ║ {:0>8b} ║
║ F: {:0>2X} ║ {:0>8b} ║
║ H: {:0>2X} ║ {:0>8b} ║
║ L: {:0>2X} ║ {:0>8b} ║
╚═══════╩══════════╝
╔══════════╦══════════════════╗
║ PC: {:0>4X} ║ {:0>16b} ║
║ SP: {:0>4X} ║ {:0>16b} ║
╚══════════╩══════════════════╝", 
        self.register['A' as usize], self.register['A' as usize],
        self.register['B' as usize], self.register['B' as usize],
        self.register['C' as usize], self.register['C' as usize],
        self.register['D' as usize], self.register['D' as usize],
        self.register['E' as usize], self.register['E' as usize],
        self.register['F' as usize], self.register['F' as usize],
        self.register['H' as usize], self.register['H' as usize],
        self.register['L' as usize], self.register['L' as usize],
        self.pc, self.pc,
        self.sp, self.sp);
        
    }

}

use crate::instructions;

pub struct Cpu {
    pub registers: Registers,
    pub instructions: json::JsonValue,
}

impl Cpu {
    pub fn new() -> Self {
        
        Cpu {
            registers: Registers::new(), 
            instructions: instructions::new(),
        }
    }
    pub fn execute_instruction(&mut self,  rom: &mut crate::Rom) -> i32 {
        let byte = rom.read_byte(self);
        let byte_as_string = format!("0x{:0>2x}",byte);
        let instruction_json = &instructions::new()["unprefixed"][byte_as_string];
        println!("Opcode for Byte: 0x{:0>2X} is {}", byte, instruction_json);
        //TODO
        match &*instruction_json["mnemonic"].to_string() {
            "LD" => instructions::LD(&instruction_json, rom, self),
            "PREFIX" => self.execute_prefix_instruction(rom),
            _=> self.instruction_unimplemented(instruction_json),
        }
        //TODO return cycles used
        
    }

    fn execute_prefix_instruction(&mut self, rom: &mut crate::Rom) -> i32{
        let byte = rom.read_byte(self);
        let byte_as_string = format!("0x{:0>2x}",byte);
        let instruction_json = &self.instructions["cbprefixed"][byte_as_string];
        println!("Opcode for Byte: 0x{:0>2X} is {}", byte, instruction_json);
        //TODO return cycles
        panic!("PLEASE REMEMBER TODO: RETURN CORRECT CYCLES, BEFORE DELETING THIS PANIC MESSAGE");
        8
    }

    fn instruction_unimplemented(&self, instruction_json: &json::JsonValue) -> i32{
        panic!("Unimplemented {}, addr: {}", instruction_json["mnemonic"], instruction_json["addr"]);
        
    }


}
