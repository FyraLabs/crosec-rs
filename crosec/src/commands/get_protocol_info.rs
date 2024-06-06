use std::{fs::File, mem::size_of, os::fd::AsRawFd};

use bytemuck::{Pod, Zeroable};

use crate::{commands::CrosEcCmd, ec_command::ec_command_bytemuck, EcCmdResult};

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct EcResponseGetProtocolInfo {
    protocol_versions: u32,
    max_request_packet_size: u16,
    max_response_packet_size: u16,
    flags: u32,
}

impl EcResponseGetProtocolInfo {
    pub fn max_ec_input_size(&self) -> usize {
        self.max_request_packet_size as usize - size_of::<EcHostRequest>()
    }

    pub fn max_ec_output_size(&self) -> usize {
        self.max_response_packet_size as usize - size_of::<EcHostResponse>()
    }
}

#[repr(C)]
struct EcHostRequest {
    struct_version: u8,
    checksum: u8,
    command: u16,
    command_version: u8,
    reserved: u8,
    data_len: u16,
}

#[repr(C)]
struct EcHostResponse {
    struct_version: u8,
    checksum: u8,
    result: u16,
    data_len: u16,
    reserved: u16,
}

pub fn get_protocol_info(file: &mut File) -> EcCmdResult<EcResponseGetProtocolInfo> {
    ec_command_bytemuck(CrosEcCmd::GetProtocolInfo, 0, &(), file.as_raw_fd())
}
