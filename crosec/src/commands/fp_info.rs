use std::{fs::File, os::fd::AsRawFd};

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
impl EcResponseFpInfo {
    pub(crate) fn get_simple_image_size(&self) -> usize {
        (self.width as usize) * (self.height as usize) * (self.bpp as usize) / 8
    }
}

pub fn fp_info(file: &mut File) -> EcCmdResult<EcResponseFpInfo> {
    let fd = file.as_raw_fd();
    let versions = ec_cmd_get_cmd_versions(file, CrosEcCmd::FpInfo)?;
    if versions & V1 == 0 {
        panic!("fp doesn't support V1. Other versions are currently not implemented");
    }
    let info: EcResponseFpInfo = ec_command_bytemuck(CrosEcCmd::FpInfo, 1, &(), fd)?;
    Ok(info)
}
