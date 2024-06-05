use std::ffi::c_int;

use bytemuck::{Pod, Zeroable};

use crate::{ec_command::ec_command_bytemuck, EcCmdResult};

use super::{
    get_cmd_versions::{ec_cmd_get_cmd_versions, V1},
    CrosEcCmd,
};

#[repr(C, align(4))]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct EcResponseFpInfo {
    pub vendor_id: u32,
    pub product_id: u32,
    pub model_id: u32,
    pub version: u32,
    pub frame_size: u32,
    pub pixel_format: u32,
    pub width: u16,
    pub height: u16,
    pub bpp: u16,
    pub errors: u16,
    pub template_size: u32,
    pub template_max: u16,
    pub template_valid: u16,
    pub template_dirty: u32,
    pub template_version: u32,
}

pub fn fp_info(fd: c_int) -> EcCmdResult<EcResponseFpInfo> {
    let versions = ec_cmd_get_cmd_versions(fd, CrosEcCmd::FpInfo)?;
    if versions & V1 == 0 {
        panic!("fp doesn't support V1. Other versions are currently not implemented");
    }
    let info: EcResponseFpInfo = ec_command_bytemuck(CrosEcCmd::FpInfo, 1, &(), fd)?;
    Ok(info)
}
