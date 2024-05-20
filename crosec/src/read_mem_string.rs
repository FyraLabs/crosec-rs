use std::ffi::c_int;
use crate::commands::read_mem::ec_cmd_read_mem;
use crate::EC_MEM_MAP_MAX_TEXT_SIZE;

pub fn read_mem_string(fd: c_int, offset: u8) -> Result<String, c_int> {
   let string =  ec_cmd_read_mem(fd, offset as u32, EC_MEM_MAP_MAX_TEXT_SIZE as u32)?;
    Ok(String::from_utf8(string).unwrap())
}
