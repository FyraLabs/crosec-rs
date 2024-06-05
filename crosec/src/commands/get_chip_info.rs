use bytemuck::{Pod, Zeroable};
use nix::libc::c_int;

use crate::{commands::CrosEcCmd, ec_command::ec_command_bytemuck, EcCmdResult};

#[repr(C, align(4))]
#[derive(Pod, Zeroable, Copy, Clone)]
struct EcResponseGetChipInfo {
    vendor: [u8; 32],
    name: [u8; 32],
    revision: [u8; 32],
}

pub fn ec_cmd_get_chip_info(fd: c_int) -> EcCmdResult<(String, String, String)> {
    let response: EcResponseGetChipInfo = ec_command_bytemuck(CrosEcCmd::GetChipInfo, 0, &(), fd)?;

    let vendor = String::from_utf8(response.vendor.to_vec()).unwrap_or_default();
    let name = String::from_utf8(response.name.to_vec()).unwrap_or_default();
    let revision = String::from_utf8(response.revision.to_vec()).unwrap_or_default();

    Ok((vendor, name, revision))
}
