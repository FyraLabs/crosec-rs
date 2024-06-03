use bytemuck::{Pod, Zeroable};

use crate::{commands::CrosEcCmd, ec_command, EcCmdResult, EcInterface};

#[repr(C, align(4))]
#[derive(Pod, Zeroable, Copy, Clone)]
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

    let params_slice = bytemuck::bytes_of(&params);

    let result = ec_command(CrosEcCmd::GetChipInfo, 0, params_slice, EcInterface::Dev(String::from("/dev/cros_ec")))?;
    let response = bytemuck::from_bytes::<EcResponseGetChipInfo>(&result);

    let vendor = String::from_utf8(response.vendor.to_vec()).unwrap_or_default();
    let name = String::from_utf8(response.name.to_vec()).unwrap_or_default();
    let revision = String::from_utf8(response.revision.to_vec()).unwrap_or_default();

    Ok((vendor, name, revision))
}
