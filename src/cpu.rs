
use crate::registers::Registers;
use crate::instructions;

pub struct Cpu {
    pub registers: Registers,
    pub opcodes_table: json::JsonValue,
}

impl Cpu {
    pub fn new() -> Self {
        
        Cpu {
            registers: Registers::new(),
            opcodes_table: instructions::new(),
        }
    }
    pub fn execute_instruction(&mut self,  rom: &mut crate::Rom) -> i32 {
        /*let byte = rom.read_byte(self);
        let byte_as_string = format!("0x{:0>2x}",byte);*/
        let instruction_json = &self.opcodes_table["unprefixed"][byte_as_string];
        println!("Opcode for Byte: 0x{:0>2X} is {}", byte, instruction_json);
        //TODO
        match &*instruction_json["mnemonic"].to_string() {
            "LD" => self.instruction_unimplemented(instruction_json),//instructions::LD(&instruction_json, rom, self),
            "XOR" => self.instruction_unimplemented(instruction_json),//instructions::XOR(&instruction_json, rom, self),
            "PREFIX" => self.execute_prefix_instruction(rom),
            _=> self.instruction_unimplemented(instruction_json),
        }
        //TODO return cycles used
        
    }

    fn execute_prefix_instruction(&mut self, rom: &mut crate::Rom) -> i32{
        /*
        let byte = rom.read_byte(self);
        let byte_as_string = format!("0x{:0>2x}",byte); */
        let instruction_json = &self.opcodes_table["cbprefixed"][byte_as_string];
        println!("Opcode for Byte: 0x{:0>2X} is {}", byte, instruction_json);
        //TODO return cycles
        panic!("PLEASE REMEMBER TODO: RETURN CORRECT CYCLES, BEFORE DELETING THIS PANIC MESSAGE");
        8
    }

    fn instruction_unimplemented(&self, instruction_json: &json::JsonValue) -> i32{
        panic!("Unimplemented {}, addr: {}", instruction_json["mnemonic"], instruction_json["addr"]);
        
    }


}
