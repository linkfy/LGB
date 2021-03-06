#[derive(Clone, Debug)]
pub struct Registers {
    register: Vec<u8>,
    pub sp: u16,
    pub pc: u16,
}
//Also defined in instructions file
pub const FLAG_Z:u8 = 7;
pub const FLAG_N:u8 = 6;
pub const FLAG_H:u8 = 5;
pub const FLAG_C:u8 = 4;


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
    
    //FLAGS ARE SAVED IN POSITION 0
    pub fn set_flag_bit(&mut self, n_bit: u8, val: bool) {
        let mut register_f = self.get("0");
        // (tu_u8 & (0x1 << n)) == (0x1 << n)
        let bit = (0x1 << n_bit) as u8;

        match val {
            true => register_f |= bit,
            false => register_f &= !bit
        }

        self.set("0", register_f as u16);
    }

    pub fn get_flag_bit(&self, n_bit: u8) -> u8 {
        let mut register_f = self.get("0");
        // (tu_u8 & (0x1 << n)) == (0x1 << n)
        let bit = (0x1 << n_bit) as u8;
        //Select the bit
        let result = (register_f & bit);
        /*
            example:            1111
            bit to analyze:     0100    & operation:
            Res:                0100

            If result is not 0 the flag is active
        */

        if(result != 0) {
            0x1
        } else {
            0x0
        }
        
    }

    //Works also for flags
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

    pub fn get(&self, reg: &str) -> u8 {
        let ch = reg.chars().next().unwrap().to_ascii_uppercase();
        let result = self.register[ch as usize];
        result
    }

    pub fn print(&self) {
        //The Value of F is the value of ZNHC FLAGS so when we are printing it we need to print the real value:
        let mut real_register_F: u8 = 0;
        let real_register_F = real_register_F | (self.get_flag_bit(FLAG_Z)<<7);
        let real_register_F = real_register_F | (self.get_flag_bit(FLAG_N)<<6);
        let real_register_F = real_register_F | (self.get_flag_bit(FLAG_H)<<5);
        let real_register_F = real_register_F | (self.get_flag_bit(FLAG_C)<<4);


        println!("
╔═══════╦══════════╗ ╔══════════╦══════════════════╗
║ A: {:0>2X} ║ {:0>8b} ║ ║ PC: {:0>4X} ║ {:0>16b} ║
║ B: {:0>2X} ║ {:0>8b} ║ ║ SP: {:0>4X} ║ {:0>16b} ║
║ C: {:0>2X} ║ {:0>8b} ║ ╚══════════╩══════════════════╝
║ D: {:0>2X} ║ {:0>8b} ║ ╔══════╗
║ E: {:0>2X} ║ {:0>8b} ║ ║ Z: {:?} ║
║ F: {:0>2X} ║ {:0>8b} ║ ║ N: {:?} ║
║ H: {:0>2X} ║ {:0>8b} ║ ║ H: {:?} ║
║ L: {:0>2X} ║ {:0>8b} ║ ║ C: {:?} ║
╚═══════╩══════════╝ ╚══════╝", 
        self.register['A' as usize], self.register['A' as usize], self.pc, self.pc,
        self.register['B' as usize], self.register['B' as usize], self.sp, self.sp,
        self.register['C' as usize], self.register['C' as usize],
        self.register['D' as usize], self.register['D' as usize], 
        self.register['E' as usize], self.register['E' as usize], self.get_flag_bit(FLAG_Z),
        real_register_F, real_register_F, self.get_flag_bit(FLAG_N),
        self.register['H' as usize], self.register['H' as usize], self.get_flag_bit(FLAG_H),
        self.register['L' as usize], self.register['L' as usize], self.get_flag_bit(FLAG_C));
    }

}