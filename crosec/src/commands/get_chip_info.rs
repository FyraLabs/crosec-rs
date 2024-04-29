use crate::{commands::CrosEcCmd, ec_command, EcInterface, EcCmdResult};
use std::mem::size_of;
use std::slice;

#[repr(C, align(4))]
struct EcResponseGetChipInfo {
    vendor: [u8; 32],
    name: [u8; 32],
    revision: [u8; 32],
}

pub fn ec_cmd_get_chip_info() -> EcCmdResult<(String, String, String)> {
    let params = EcResponseGetChipInfo {
        vendor: [0; 32],
        name: [0; 32],
        revision: [0; 32],
    };

    let params_ptr = &params as *const _ as *const u8;
    let params_slice =
        unsafe { slice::from_raw_parts(params_ptr, size_of::<EcResponseGetChipInfo>()) };

    let result = ec_command(CrosEcCmd::GetChipInfo, 0, params_slice, EcInterface::Dev(String::from("/dev/cros_ec")))?;
    let response: EcResponseGetChipInfo = unsafe { std::ptr::read(result.as_ptr() as *const _) };

    let vendor = String::from_utf8(response.vendor.to_vec()).unwrap_or(String::from(""));
    let name = String::from_utf8(response.name.to_vec()).unwrap_or(String::from(""));
    let revision = String::from_utf8(response.revision.to_vec()).unwrap_or(String::from(""));

    Ok((vendor, name, revision))
}
