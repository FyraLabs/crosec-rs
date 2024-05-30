use bytemuck::{Pod, Zeroable};
use nix::libc::c_int;

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command;
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

pub fn ec_cmd_get_cmd_versions(fd: c_int, cmd: CrosEcCmd) -> EcCmdResult<u32> {
    let result = match ec_command(CrosEcCmd::GetCmdVersions, 1, bytemuck::bytes_of(&EcParamsGetCmdVersionV1 {
        cmd: cmd as u16
    }), fd) {
        Ok(response) => {
            Ok(response)
        },
        Err(_e) => {
            ec_command(CrosEcCmd::GetCmdVersions, 0, bytemuck::bytes_of(&EcParamsGetCmdVersionV0 {
                cmd: cmd as u8
            }), fd)
        }
    }?;
    let response = bytemuck::from_bytes::<EcResponseGetCmdVersion>(&result);
    Ok(response.version_mask)
}