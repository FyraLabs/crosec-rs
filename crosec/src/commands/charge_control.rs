use crate::commands::get_cmd_versions::{ec_cmd_get_cmd_versions, V2};
use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command;
use crate::EcCmdResult;
use bytemuck::{Pod, Zeroable};
use std::ffi::c_int;

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default, Debug)]
pub struct Sustainer {
    pub min_percent: i8,
    pub max_percent: i8,
}

#[derive(Debug)]
pub enum ChargeControl {
    Normal(Option<Sustainer>),
    Idle,
    Discharge,
}

pub fn supports_get_and_sustainer(fd: c_int) -> EcCmdResult<bool> {
    let versions = ec_cmd_get_cmd_versions(fd, CrosEcCmd::ChargeControl)?;
    Ok(versions & V2 != 0)
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub struct EcParamsChargeControl {
    mode: u32,
    command: u8,
    reserved: u8,
    sustain: Sustainer,
}

const CHARGE_CONTROL_MODE_SET: u8 = 0;
// const CHARGE_CONTROL_MODE_GET: u8 = 1;

const CHARGE_CONTROL_COMMAND_NORMAL: u32 = 0;
const CHARGE_CONTROL_COMMAND_IDLE: u32 = 1;
const CHARGE_CONTROL_COMMAND_DISCHARGE: u32 = 2;

pub fn get_charge_control(_fd: c_int) -> EcCmdResult<ChargeControl> {
    panic!("Not implemented yet");
}

pub fn set_charge_control(fd: c_int, charge_control: ChargeControl) -> EcCmdResult<()> {
    let params = EcParamsChargeControl {
        command: CHARGE_CONTROL_MODE_SET,
        mode: match charge_control {
            ChargeControl::Normal(_) => CHARGE_CONTROL_COMMAND_NORMAL,
            ChargeControl::Idle => CHARGE_CONTROL_COMMAND_IDLE,
            ChargeControl::Discharge => CHARGE_CONTROL_COMMAND_DISCHARGE,
        },
        reserved: Default::default(),
        sustain: match charge_control {
            ChargeControl::Normal(sustain) => sustain.unwrap_or(Sustainer {
                min_percent: -1,
                max_percent: -1,
            }),
            _ => Default::default(),
        },
    };
    ec_command(
        CrosEcCmd::ChargeControl,
        {
            let version = ec_cmd_get_cmd_versions(fd, CrosEcCmd::ChargeControl)?;
            Ok(if version & V2 != 0 { 2 } else { 1 })
        }?,
        bytemuck::bytes_of(&params),
        fd,
    )?;
    Ok(())
}
