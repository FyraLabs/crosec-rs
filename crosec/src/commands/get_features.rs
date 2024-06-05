use nix::libc::c_int;

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command_bytemuck;
use crate::EcCmdResult;

pub const EC_FEATURE_PWM_FAN: u64 = 0b100;

pub fn ec_cmd_get_features(fd: c_int) -> EcCmdResult<u64> {
    ec_command_bytemuck(CrosEcCmd::GetFeatures, 0, &(), fd)
}
