use std::fs::File;
use std::os::fd::AsRawFd;

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command_bytemuck;
use crate::EcCmdResult;

pub const EC_FEATURE_PWM_FAN: u64 = 0b100;

pub fn ec_cmd_get_features(file: &mut File) -> EcCmdResult<u64> {
    ec_command_bytemuck(CrosEcCmd::GetFeatures, 0, &(), file.as_raw_fd())
}
