use std::os::fd::AsRawFd;

use bytemuck::{Pod, Zeroable};

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command_bytemuck;
use crate::EcCmdResult;

#[repr(C)]
#[derive(Pod, Copy, Clone, Zeroable)]
struct EcParamsADCRead {
    adc_channel: u8,
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Zeroable)]
struct EcResponseADCRead {
    adc_value: i32,
}

pub fn ec_cmd_adc_read<File: AsRawFd>(file: &mut File, channel: u8) -> EcCmdResult<i32> {
    let fd = file.as_raw_fd();
    let response: EcResponseADCRead = ec_command_bytemuck(
        CrosEcCmd::ADCRead,
        0,
        &EcParamsADCRead {
            adc_channel: channel,
        },
        fd,
    )?;
    Ok(response.adc_value)
}
