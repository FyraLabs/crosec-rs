use std::os::fd::AsRawFd;

use bytemuck::{Pod, Zeroable};

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command_bytemuck;
use crate::EcCmdResult;

#[repr(C)]
#[derive(Pod, Copy, Clone, Zeroable)]
struct EcParamsGetCmdVersionV1 {
    cmd: u16,
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
struct EcParamsGetCmdVersionV0 {
    cmd: u8,
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
struct EcResponseGetCmdVersion {
    version_mask: u32,
}

pub const V0: u32 = 0b001;
pub const V1: u32 = 0b010;
pub const V2: u32 = 0b100;

pub fn ec_cmd_get_cmd_versions<File: AsRawFd>(file: &mut File, cmd: CrosEcCmd) -> EcCmdResult<u32> {
    let fd = file.as_raw_fd();
    let response: EcResponseGetCmdVersion = match ec_command_bytemuck(
        CrosEcCmd::GetCmdVersions,
        1,
        &EcParamsGetCmdVersionV1 { cmd: cmd as u16 },
        fd,
    ) {
        Ok(response) => Ok(response),
        Err(_e) => ec_command_bytemuck(
            CrosEcCmd::GetCmdVersions,
            0,
            &EcParamsGetCmdVersionV0 { cmd: cmd as u8 },
            fd,
        ),
    }?;
    Ok(response.version_mask)
}
