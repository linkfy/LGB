extern crate json;
pub const FLAG_Z:u8 = 7;
pub const FLAG_N:u8 = 6;
pub const FLAG_H:u8 = 5;
pub const FLAG_C:u8 = 4;

pub fn new() -> json::JsonValue {
  crate::opcodes::new()
}