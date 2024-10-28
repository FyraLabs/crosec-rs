use std::{fs::File, os::fd::AsRawFd};

use bytemuck::{Pod, Zeroable};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{
    commands::CrosEcCmd,
    ec_command::{ec_command_bytemuck, ec_command_with_dynamic_output_size},
    EcCmdResult,
};

use super::get_protocol_info::EcResponseGetProtocolInfo;

const TOOLVERSION: &str = env!("CARGO_PKG_VERSION");

#[repr(C, align(4))]
#[derive(Pod, Zeroable, Copy, Clone)]
struct EcResponseVersionV1 {
    version_string_ro: [u8; 32],
    version_string_rw: [u8; 32],
    cros_fwid_ro: [u8; 32],
    current_image: u32,
    cros_fwid_rw: [u8; 32],
}

#[derive(FromPrimitive)]
enum EcImage {
    Unknown = 0,
    Ro = 1,
    Rw = 2,
    RoB = 3,
    RwB = 4,
}

pub fn ec_cmd_version(
    file: &mut File,
    protocol_info: &EcResponseGetProtocolInfo,
) -> EcCmdResult<(String, String, String, String, String)> {
    let params = EcResponseVersionV1 {
        version_string_ro: [0; 32],
        version_string_rw: [0; 32],
        cros_fwid_ro: [0; 32],
        current_image: 0,
        cros_fwid_rw: [0; 32],
    };

    let response: EcResponseVersionV1 =
        ec_command_bytemuck(CrosEcCmd::Version, 0, &params, file.as_raw_fd())?;

    let ro_ver = String::from_utf8(response.version_string_ro.to_vec()).unwrap_or_default();
    let rw_ver = String::from_utf8(response.version_string_rw.to_vec()).unwrap_or_default();

    let image = match FromPrimitive::from_u32(response.current_image) {
        Some(EcImage::Unknown) => String::from("Unknown"),
        Some(EcImage::Ro) => String::from("RO"),
        Some(EcImage::Rw) => String::from("RW"),
        Some(EcImage::RoB) => String::from("RO B"),
        Some(EcImage::RwB) => String::from("RW B"),
        None => String::from("Unknown"),
    };

    let result = ec_command_with_dynamic_output_size(
        CrosEcCmd::GetBuildInfo,
        0,
        &[0; 248],
        protocol_info.max_ec_output_size(),
        file.as_raw_fd(),
    )?;

    let build_info = String::from_utf8(result).unwrap_or(String::from(""));
    Ok((ro_ver, rw_ver, image, build_info, String::from(TOOLVERSION)))
}
