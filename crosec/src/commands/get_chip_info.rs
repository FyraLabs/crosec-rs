use std::{fs::File, os::fd::AsRawFd};

use bytemuck::{Pod, Zeroable};

use crate::{commands::CrosEcCmd, ec_command::ec_command_bytemuck, EcCmdResult};

#[repr(C, align(4))]
#[derive(Pod, Zeroable, Copy, Clone)]
struct EcResponseGetChipInfo {
    vendor: [u8; 32],
    name: [u8; 32],
    revision: [u8; 32],
}

pub fn ec_cmd_get_chip_info(file: &mut File) -> EcCmdResult<(String, String, String)> {
    let response: EcResponseGetChipInfo =
        ec_command_bytemuck(CrosEcCmd::GetChipInfo, 0, &(), file.as_raw_fd())?;

    let vendor = String::from_utf8(response.vendor.to_vec()).unwrap_or_default();
    let name = String::from_utf8(response.name.to_vec()).unwrap_or_default();
    let revision = String::from_utf8(response.revision.to_vec()).unwrap_or_default();

    Ok((vendor, name, revision))
}
