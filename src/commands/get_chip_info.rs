use crate::commands::CrosEcCmds;
use crate::crosec::dev::ec_command;
use std::mem::size_of;
use std::slice;

#[repr(C, align(4))]
struct EcResponseGetChipInfo {
    vendor: [u8; 32],
    name: [u8; 32],
    revision: [u8; 32],
}

pub fn ec_cmd_get_chip_info() -> (String, String, String) {
    let params = EcResponseGetChipInfo {
        vendor: [0; 32],
        name: [0; 32],
        revision: [0; 32],
    };

    let params_ptr = &params as *const _ as *const u8;
    let params_slice = unsafe { slice::from_raw_parts(params_ptr, size_of::<EcResponseGetChipInfo>()) };

    let result = ec_command(CrosEcCmds::GetChipInfo as u32, 0, params_slice)
        .unwrap_or_else(|error| panic!("EC error: {error:?}"));
    let response: EcResponseGetChipInfo = unsafe { std::ptr::read(result.as_ptr() as *const _) };

    let vendor = String::from_utf8(response.vendor.to_vec()).unwrap_or(String::from(""));
    let name = String::from_utf8(response.name.to_vec()).unwrap_or(String::from(""));
    let revision = String::from_utf8(response.revision.to_vec()).unwrap_or(String::from(""));

    (vendor, name, revision)
}
