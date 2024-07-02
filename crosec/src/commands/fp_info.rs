use std::os::fd::AsRawFd;

use bytemuck::{Pod, Zeroable};

use crate::{ec_command::ec_command_bytemuck, EcCmdResult};

use super::{
    CrosEcCmd,
    get_cmd_versions::{ec_cmd_get_cmd_versions, V1},
};

#[repr(C, align(4))]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct EcResponseFpInfo {
    pub vendor_id: u32,
    pub product_id: u32,
    pub model_id: u32,
    pub version: u32,
    /// The size of the PGM image, in bytes
    pub frame_size: u32,
    pub pixel_format: u32,
    pub width: u16,
    pub height: u16,
    pub bpp: u16,
    pub errors: u16,
    /// The template size, in bytes
    pub template_size: u32,
    /// The maximum number of templates the FP can store and match at once
    pub template_max: u16,
    /// The number of templates loaded into the FP
    pub template_valid: u16,
    /// The first bit (the rightmost) represents template 0, the 2nd bit form the right represents template 1, etc.
    /// If the bit is 1, that means that the template has been updated by the FP and the updated version has not been downloaded yet.
    pub template_dirty: u32,
    /// This version could increase after an update to the FP firmware
    pub template_version: u32,
}
impl EcResponseFpInfo {
    pub(crate) fn get_simple_image_size(&self) -> usize {
        (self.width as usize) * (self.height as usize) * (self.bpp as usize) / 8
    }
}

pub fn fp_info<File: AsRawFd>(file: &mut File) -> EcCmdResult<EcResponseFpInfo> {
    let fd = file.as_raw_fd();
    let versions = ec_cmd_get_cmd_versions(file, CrosEcCmd::FpInfo)?;
    if versions & V1 == 0 {
        panic!("fp doesn't support V1. Other versions are currently not implemented");
    }
    let info: EcResponseFpInfo = ec_command_bytemuck(CrosEcCmd::FpInfo, 1, &(), fd)?;
    Ok(info)
}
