use crate::{commands::CrosEcCmd, ec_command, EcInterface, EcCmdResult};
use crate::dev::BUF_SIZE;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::mem::size_of;
use std::slice;

const TOOLVERSION: &str = env!("CARGO_PKG_VERSION");

#[repr(C, align(4))]
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

pub fn ec_cmd_version() -> EcCmdResult<(String, String, String, String, String)> {
    let params = EcResponseVersionV1 {
        version_string_ro: [0; 32],
        version_string_rw: [0; 32],
        cros_fwid_ro: [0; 32],
        current_image: 0,
        cros_fwid_rw: [0; 32],
    };

    let build_string: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let params_ptr = &params as *const _ as *const u8;
    let params_slice =
        unsafe { slice::from_raw_parts(params_ptr, size_of::<EcResponseVersionV1>()) };

    let result = ec_command(CrosEcCmd::Version, 0, params_slice, EcInterface::Dev(String::from("/dev/cros_ec")))?;
    let response: EcResponseVersionV1 = unsafe { std::ptr::read(result.as_ptr() as *const _) };

    let ro_ver = String::from_utf8(response.version_string_ro.to_vec()).unwrap_or(String::from(""));
    let rw_ver = String::from_utf8(response.version_string_rw.to_vec()).unwrap_or(String::from(""));

    let image = match FromPrimitive::from_u32(response.current_image) {
        Some(EcImage::Unknown) => String::from("Unknown"),
        Some(EcImage::Ro) => String::from("RO"),
        Some(EcImage::Rw) => String::from("RW"),
        Some(EcImage::RoB) => String::from("RO B"),
        Some(EcImage::RwB) => String::from("RW B"),
        None => String::from("Unknown"),
    };

    let build_string_ptr = &build_string as *const _ as *const u8;
    let build_string_slice = unsafe { slice::from_raw_parts(build_string_ptr, BUF_SIZE) };

    let result = ec_command(CrosEcCmd::GetBuildInfo, 0, build_string_slice, EcInterface::Dev(String::from("/dev/cros_ec")))?;
    let response: [u8; BUF_SIZE] = unsafe { std::ptr::read(result.as_ptr() as *const _) };

    let build_info = String::from_utf8(response.to_vec()).unwrap_or(String::from(""));
    Ok((ro_ver, rw_ver, image, build_info, String::from(TOOLVERSION)))
}
