pub fn LD(instruction_json: &json::JsonValue, rom: &mut crate::Rom, cpu: &mut crate::Cpu) -> i32 {
    /* 
    
      1 char B
      2 char d8
      3 char d16
      4 char (HL)
      5 char (HL+) / (HL-) / (a16) / SP+r8 
      
    */
    
    
      //println!("EXECUTING: {}", instruction_json);
    
      let operand1 = &instruction_json["operand1"].as_str().unwrap();
      let operand2 = &instruction_json["operand2"].as_str().unwrap();
    
      if operand2 == &"d16" {
        let byte1 = rom.read_byte(cpu);
        let byte2 = rom.read_byte(cpu);
        //Change order because little endian
        let d16: u16 = ((byte2 as u16) << 8) | byte1 as u16;
        
        cpu.registers.set(operand1, d16);
    
      } else if operand2 == &"d8" {
    
        if operand1 == &"(HL)" {
          
          let byte_rom = rom.read_byte(cpu);
          let byte1 = cpu.registers.get("H");
          let byte2 = cpu.registers.get("L");
          let d16: u16 = ((byte2 as u16) << 8) | byte1 as u16;
          
          
        } else {
          //Operand1 = B,D,H,C,E,L,A
          let byte_rom = rom.read_byte(cpu);
          cpu.registers.set(operand1, byte_rom as u16);
        }
    
      }
      else {
        panic!("This LD option is not implemented yet");
      }
    
      let cycles = &instruction_json["cycles"][0];
      let cycles2 = &instruction_json["cycles"][1];
    
      if !cycles2.is_null() {
        panic!("WE DID NOT IMPLEMENTED YET THIS FUNCTION WITH MULTICICLE DECISION")
      } else {
        cycles.as_i32().unwrap()
      }
    }
    
    pub fn XOR(instruction_json: &json::JsonValue, rom: &mut crate::Rom, cpu: &mut crate::Cpu) -> i32 {
      
      let operand1 = &instruction_json["operand1"].as_str().unwrap();
      let first_char = operand1.chars().next().unwrap();
      if first_char == '(' {
        panic!("Unimplemented Address modes (HL)");
      }
      let register = cpu.registers.get(operand1);
      
      let register = register ^ register;
      
      cpu.registers.set("A", register as u16);
      
      //set FLAGS
      cpu.registers.set_flag_bit(FLAG_Z, true);
      cpu.registers.set_flag_bit(FLAG_N, false);
      cpu.registers.set_flag_bit(FLAG_H, false);
      cpu.registers.set_flag_bit(FLAG_C, false);
      //TODO
    
      let cycles = &instruction_json["cycles"][0];
      cycles.as_i32().unwrap()
      
    }