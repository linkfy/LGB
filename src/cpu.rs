use crate::registers::Registers;
use crate::instructions;
use crate::memory_bus::Memory;

pub struct Cpu {
    pub registers: Registers,
    pub opcodes_table: json::JsonValue,
}

#[derive(Debug)]
pub struct InstructionData {
    pub mnemonic: String,
    pub length: i32,
    pub cycles: Vec<i32>,
    pub flags: [char; 4],
    pub addr: String,
    pub operands: Vec<String>
}

impl Cpu {
    pub fn new() -> Self {
        
        Cpu {
            registers: Registers::new(),
            opcodes_table: instructions::new(),
        }
    }

    fn parse_instruction(&self, instruction_json: &json::JsonValue) -> InstructionData {
        //println!("{}", instruction_json);
        let mnemonic = instruction_json["mnemonic"].to_string();
        let length = instruction_json["length"].to_string().parse::<i32>().unwrap();
        let mut cycles =  Vec::new();
        //copy one if only have one cycle, or two if more
        if instruction_json["cycles"][1].is_null() {
            let cycles1: i32 = instruction_json["cycles"][0].to_string().parse::<i32>().unwrap();
            cycles.push(cycles1);
        } else {
            let cycles1: i32 = instruction_json["cycles"][0].to_string().parse::<i32>().unwrap();
            let cycles2: i32 = instruction_json["cycles"][1].to_string().parse::<i32>().unwrap();
            cycles.push(cycles1);
            cycles.push(cycles2);
        }
        
        let flag1: char = instruction_json["flags"][0].to_string().chars().next().unwrap();
        let flag2: char = instruction_json["flags"][1].to_string().chars().next().unwrap();
        let flag3: char = instruction_json["flags"][2].to_string().chars().next().unwrap();
        let flag4: char = instruction_json["flags"][3].to_string().chars().next().unwrap();
        let mut flags = [flag1, flag2, flag3, flag4];
        let addr = instruction_json["addr"].to_string();
        let mut operands = Vec::new();
        //Copy one if one operand or two if more
        if instruction_json["operand2"].is_null() {
            let op1: String = instruction_json["operand1"].to_string();
            operands.push(op1);
        } else {
            let op1: String = instruction_json["operand1"].to_string();
            let op2: String = instruction_json["operand2"].to_string();
            operands.push(op1);
            operands.push(op2);
        }

        InstructionData {
            mnemonic: mnemonic,
            length: length,
            cycles: cycles,
            flags: flags,
            addr: addr,
            operands: operands
        }

    }

    
    /// Return the number of cycles passed
    pub fn execute_instruction(&mut self,  mem: &mut Memory) -> i32 {
        let byte = mem.read_byte(self);
        let byte_as_string = format!("0x{:0>2x}",byte);
        let instruction_json = &self.opcodes_table["unprefixed"][byte_as_string];
        let instruction = self.parse_instruction(instruction_json);
        let instruction_cycles = i32::clone(&instruction.cycles[0]);
        println!("{:?}", instruction);
        
        

        //println!("Opcode for Byte: 0x{:0>2X} is {}", byte, instruction_json);
        //TODO
        match &*instruction_json["mnemonic"].to_string() {
            "LD" => self.LD(instruction, mem),//instructions::LD(&instruction_json, rom, self),
            "XOR" => self.XOR(instruction, mem),
            //"XOR" => self.instruction_unimplemented(&instruction_json),//instructions::XOR(&instruction_json, rom, self),
            "PREFIX" => self.execute_prefix_instruction(mem),
            _=> self.instruction_unimplemented(&instruction_json),
        };
        //Return cycles used
        instruction_cycles
    }

    fn execute_prefix_instruction(&mut self, mem: &mut Memory) -> i32{
        /*
        let byte = rom.read_byte(self);
        let byte_as_string = format!("0x{:0>2x}",byte); */
        /* let instruction_json = &self.opcodes_table["cbprefixed"][byte_as_string];
        println!("Opcode for Byte: 0x{:0>2X} is {}", byte, instruction_json);
        //TODO return cycles
        panic!("PLEASE REMEMBER TODO: RETURN CORRECT CYCLES, BEFORE DELETING THIS PANIC MESSAGE");
        8 */
        0
    }

    fn instruction_unimplemented(&self, instruction_json: &json::JsonValue) -> i32{
        panic!("Unimplemented {}, addr: {}", instruction_json["mnemonic"], instruction_json["addr"]);
        
    }

    fn LD(&mut self, instruction: InstructionData, mem: &mut Memory) -> i32{
        //Case 0x31 LD "SP", "d16" && 0x21 // FLAGS are not modified
        let byte1: u8 = mem.read_byte(self);
        let byte2: u8 = mem.read_byte(self);
        //Little endian: last byte goes first in memory
        let byteJoin: u16 = (byte2 as u16) << 8 | byte1 as u16;

        //NOTE: Currently PC are set by "read memory", per each byte we increment PC but we have this info inside InstructionData too
        


        // EXAMPLE TO SET FLAGS: self.registers.set_flag_bit(crate::registers::FLAG_C, false); //Z N H C

        self.registers.set(&instruction.operands[0], byteJoin); //We need to set cycles? TODO
        0
        
    }

    fn XOR(&mut self, instruction: InstructionData, mem: &mut Memory) ->i32 {
        //All the XOR operations set A to the value of the register from operand collected
        //ALL CASES: XOR A, selected_register
        //0xAF only has 1 byte length, we only need to read one time from memory
        //let byte1: u8 = mem.read_byte(self); //PC is incremented by read byte but also we have the info inside InstructionData
        let register_A = self.registers.get("A");
        let selected_register = self.registers.get(&instruction.operands[0]);
        //println!("Operand {:}", &instruction.operands[0]);
        let result_operation = register_A ^ selected_register;
        if(result_operation == 0) {
            self.registers.set_flag_bit(crate::registers::FLAG_Z, true);
        }
        self.registers.set_flag_bit(crate::registers::FLAG_N, false); //Unset
        self.registers.set_flag_bit(crate::registers::FLAG_H, false); //Unset
        self.registers.set_flag_bit(crate::registers::FLAG_C, false); //Unset

        self.registers.set(&instruction.operands[0], result_operation as u16); //XOR reg, reg, set register A to xor itself
        0
    }


}
