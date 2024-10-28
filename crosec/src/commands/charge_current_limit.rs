use std::os::fd::AsRawFd;

use bytemuck::{Pod, Zeroable};
use uom::si::{electric_current::milliampere, f32::ElectricCurrent};

use crate::{ec_command::ec_command_bytemuck, EcCmdResult};

use super::CrosEcCmd;

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub struct EcParamsChargeCurrentLimit {
    limit: u32, // in mA
}

/// Limit the charging current. The EC command sends the charging current limit to the nearest mA.
pub fn set_charge_current_limit<File: AsRawFd>(
    file: &mut File,
    limit: ElectricCurrent,
) -> EcCmdResult<()> {
    ec_command_bytemuck(
        CrosEcCmd::ChargeCurrentLimit,
        0,
        &EcParamsChargeCurrentLimit {
            limit: limit.get::<milliampere>() as u32,
        },
        file.as_raw_fd(),
    )
}
