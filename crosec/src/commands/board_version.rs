use std::mem::size_of;
use crate::{ec_command, EcCmdResult, EcInterface};
use crate::commands::CrosEcCmd;

pub fn ec_cmd_board_version() -> EcCmdResult<u32> {
    let mut result = ec_command(CrosEcCmd::GetBoardVersion, 0, Default::default(), EcInterface::Dev(String::from("/dev/cros_ec")))?;
    result.resize(size_of::<u32>(), Default::default());
    Ok(u32::from_le_bytes(result.try_into().unwrap()))
}
