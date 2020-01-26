extern crate json;
const FLAG_Z:u8 = 7;
const FLAG_N:u8 = 6;
const FLAG_H:u8 = 5;
const FLAG_C:u8 = 4;

pub fn new() -> json::JsonValue {
  crate::opcodes::new()
}