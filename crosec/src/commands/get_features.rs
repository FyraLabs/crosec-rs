use bytemuck::{Pod, Zeroable};
use nix::libc::c_int;

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command;
use crate::EcCmdResult;

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
struct EcResponseGetFeatures {
    flags: u64,
}

pub const EC_FEATURE_PWM_FAN: u64 = 0b100;

pub fn ec_cmd_get_features(fd: c_int) -> EcCmdResult<u64> {
    let response = ec_command(CrosEcCmd::GetFeatures, 0, Default::default(), fd)?;
    let response = bytemuck::from_bytes::<EcResponseGetFeatures>(&response);
    Ok(response.flags)
}