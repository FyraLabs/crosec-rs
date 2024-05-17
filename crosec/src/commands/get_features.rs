use bytemuck::{Pod, Zeroable};
use crate::{ec_command, EcCmdResult, EcInterface};
use crate::commands::CrosEcCmd;

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
struct EcResponseGetFeatures {
    flags: u64,
}

pub fn ec_cmd_get_features() -> EcCmdResult<u64> {
    let response = ec_command(CrosEcCmd::GetFeatures, 0, Default::default(), EcInterface::Default)?;
    let response = bytemuck::from_bytes::<EcResponseGetFeatures>(&response);
    Ok(response.flags)
}