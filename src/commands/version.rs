use crate::commands::CrosEcCmds;
use crate::crosec::dev::ec_command;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use std::mem::size_of;
use std::slice;
use std::str;

const BUILDMAX: usize = 248;
const TOOLVERSION: &str = "0.1.0";

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
	EcImageUnknown = 0,
	EcImageRo = 1,
	EcImageRw = 2,
	EcImageRoB = 3,
	EcImageRwB = 4,
}

pub fn ec_cmd_version() {
    let params = EcResponseVersionV1 {
        version_string_ro: [0; 32],
        version_string_rw: [0; 32],
        cros_fwid_ro: [0; 32],
        current_image: 0,
        cros_fwid_rw: [0; 32],
    };
    let build_string: [u8; BUILDMAX] = [0; BUILDMAX];
    let params_ptr = &params as *const _ as *const u8;
    let params_slice = unsafe { slice::from_raw_parts(params_ptr, size_of::<EcResponseVersionV1>()) };

    let result = ec_command(CrosEcCmds::Version as u32, 0, params_slice)
        .unwrap_or_else(|error| panic!("EC error: {error:?}"));
    let response: EcResponseVersionV1 = unsafe { std::ptr::read(result.as_ptr() as *const _) };
    println!("RO version:    {}", str::from_utf8(&response.version_string_ro).unwrap_or(""));
    println!("RW version:    {}", str::from_utf8(&response.version_string_rw).unwrap_or(""));
    //println!("RO CROS FWID: {}", str::from_utf8(&response.cros_fwid_ro).unwrap_or(""));
    //println!("RW CROS FWID: {}", str::from_utf8(&response.cros_fwid_rw).unwrap_or(""));
    let image =
    match FromPrimitive::from_u32(response.current_image) {
        Some(EcImage::EcImageUnknown) => String::from("Unknown"),
        Some(EcImage::EcImageRo) => String::from("RO"),
        Some(EcImage::EcImageRw) => String::from("RW"),
        Some(EcImage::EcImageRoB) => String::from("RO B"),
        Some(EcImage::EcImageRwB) => String::from("RW B"),
        None => String::from("Unknown"),
    };
    println!("Firmware copy: {image}");
    
    let build_string_ptr = &build_string as *const _ as *const u8;
    let build_string_slice = unsafe { slice::from_raw_parts(build_string_ptr, BUILDMAX) };
    let result = ec_command(CrosEcCmds::GetBuildInfo as u32, 0, build_string_slice)
    .unwrap_or_else(|error| panic!("EC error: {error:?}"));
    let response: [u8; BUILDMAX] = unsafe { std::ptr::read(result.as_ptr() as *const _) };
    println!("Build info:    {}", str::from_utf8(&response).unwrap_or(""));
    println!("Tool version:  {TOOLVERSION}");
}
