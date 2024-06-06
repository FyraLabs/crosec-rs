use crate::commands::read_mem::ec_cmd_read_mem;
use crate::EC_MEM_MAP_MAX_TEXT_SIZE;
use std::ffi::c_int;
use std::fs::File;

pub fn read_mem_string(file: &mut File, offset: u8) -> Result<String, c_int> {
    let string = ec_cmd_read_mem(file, offset as u32, EC_MEM_MAP_MAX_TEXT_SIZE as u32)?;
    Ok(String::from_utf8(string).unwrap())
}
