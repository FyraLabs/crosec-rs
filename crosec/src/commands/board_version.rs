use std::ffi::c_int;
use std::mem::size_of;

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command;
use crate::EcCmdResult;

pub fn ec_cmd_board_version(fd: c_int) -> EcCmdResult<u32> {
    let mut result = ec_command(CrosEcCmd::GetBoardVersion, 0, Default::default(), fd)?;
    result.resize(size_of::<u32>(), Default::default());
    Ok(u32::from_le_bytes(result.try_into().unwrap()))
}
