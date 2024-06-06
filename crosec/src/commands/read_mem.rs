use crate::CROS_EC_IOC_MAGIC;
use bytemuck::{Pod, Zeroable};
use nix::ioctl_readwrite;
use std::{ffi::c_int, fs::File, os::fd::AsRawFd};

const EC_MEM_MAP_SIZE: usize = 255;

#[repr(C, align(1))]
#[derive(Pod, Zeroable, Copy, Clone)]
struct EcParamsReadMem {
    offset: u8,
    size: u8,
}

#[repr(C)]
struct EcResponseReadMemV2 {
    offset: u32,
    bytes: u32,
    buffer: [u8; EC_MEM_MAP_SIZE],
}

ioctl_readwrite!(cros_ec_read_mem, CROS_EC_IOC_MAGIC, 1, EcResponseReadMemV2);

pub fn ec_cmd_read_mem(file: &mut File, offset: u32, bytes: u32) -> Result<Vec<u8>, c_int> {
    let mut response = EcResponseReadMemV2 {
        offset,
        bytes,
        buffer: [0; EC_MEM_MAP_SIZE],
    };
    let status = unsafe { cros_ec_read_mem(file.as_raw_fd(), &mut response) }.unwrap();
    if status >= 0 {
        Ok(response.buffer[..bytes as usize].to_vec())
    } else {
        Err(status)
    }
}
