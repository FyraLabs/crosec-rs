use std::ffi::c_int;
use std::fs::File;
use std::mem::size_of;

use bytemuck::AnyBitPattern;

use crate::commands::read_mem::ec_cmd_read_mem;

pub fn read_mem_any<T: AnyBitPattern>(file: &mut File, offset: u8) -> Result<T, c_int> {
    let result = ec_cmd_read_mem(file, offset as u32, size_of::<T>() as u32)?;
    let result = bytemuck::from_bytes(&result);
    Ok(*result)
}
