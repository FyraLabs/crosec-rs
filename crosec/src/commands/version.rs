use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use nix::libc::c_int;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{commands::CrosEcCmd, EcCmdResult};
use crate::ec_command::{BUF_SIZE, ec_command};

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

pub fn ec_cmd_version(fd: c_int) -> EcCmdResult<(String, String, String, String, String)> {
    let params = EcResponseVersionV1 {
        version_string_ro: [0; 32],
        version_string_rw: [0; 32],
        cros_fwid_ro: [0; 32],
        current_image: 0,
        cros_fwid_rw: [0; 32],
    };

    let build_string: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let params_slice = bytemuck::bytes_of(&params);

    let mut result = ec_command(CrosEcCmd::Version, 0, params_slice, fd)?;
    result.resize(size_of::<EcResponseVersionV1>(), Default::default());
    let response = bytemuck::from_bytes::<EcResponseVersionV1>(&result);

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

    let build_string_slice = &build_string;

    let result = ec_command(CrosEcCmd::GetBuildInfo, 0, build_string_slice, fd)?;

    let build_info = String::from_utf8(result).unwrap_or(String::from(""));
    Ok((ro_ver, rw_ver, image, build_info, String::from(TOOLVERSION)))
}
