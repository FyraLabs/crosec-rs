use std::fs::File;
use std::os::fd::AsRawFd;

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command_bytemuck;
use crate::EcCmdResult;

pub fn ec_cmd_board_version(file: &mut File) -> EcCmdResult<u32> {
    ec_command_bytemuck(CrosEcCmd::GetBoardVersion, 0, &(), file.as_raw_fd())
}
