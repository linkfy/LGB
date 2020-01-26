const RAM_SIZE: usize = 0xFFFF;

pub struct Ram {
    bytes: Vec<u8>,
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            bytes: vec![0; RAM_SIZE],
        }
    }

    pub fn set(&mut self, dir: u16, value: u8) {
        self.bytes[dir as usize] = value;
    }

    pub fn print_dir(&self, dir: u16) {
        println!("
╔═════════════════════════╗
║ Ram dir: {:0>4X}, value {:0>2X} ║
╚═════════════════════════╝", dir, self.bytes[dir as usize]);
        
    }
}