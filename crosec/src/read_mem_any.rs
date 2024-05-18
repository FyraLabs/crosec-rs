use std::ffi::c_int;
use std::mem::size_of;

use bytemuck::AnyBitPattern;

use crate::commands::read_mem::ec_cmd_read_mem;

pub fn read_mem_any<T: AnyBitPattern>(fd: c_int, offset: u8) -> Result<T, c_int> {
    let result = ec_cmd_read_mem(fd, offset as u32, size_of::<T>() as u32)?;
    let result = bytemuck::from_bytes(&result);
    Ok(*result)
}
