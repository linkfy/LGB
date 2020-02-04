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
        println!("{:?}", instruction);
        

        //println!("Opcode for Byte: 0x{:0>2X} is {}", byte, instruction_json);
        //TODO
        match &*instruction_json["mnemonic"].to_string() {
            "LD" => self.LD(instruction, mem),//instructions::LD(&instruction_json, rom, self),
            "XOR" => self.instruction_unimplemented(&instruction_json),//instructions::XOR(&instruction_json, rom, self),
            "PREFIX" => self.execute_prefix_instruction(mem),
            _=> self.instruction_unimplemented(&instruction_json),
        };
        //TODO return cycles used
        0
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

    fn LD(&self, instruction: InstructionData, mem: &mut Memory) -> i32{

        0
        
    }


}
