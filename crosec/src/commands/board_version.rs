use std::ffi::c_int;

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command_bytemuck;
use crate::EcCmdResult;

pub fn ec_cmd_board_version(fd: c_int) -> EcCmdResult<u32> {
    ec_command_bytemuck(CrosEcCmd::GetBoardVersion, 0, &(), fd)
}
