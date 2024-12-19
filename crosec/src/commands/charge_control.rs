use crate::commands::get_cmd_versions::{ec_cmd_get_cmd_versions, V2};
use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command_bytemuck;
use crate::EcCmdResult;
use bytemuck::{Pod, Zeroable};
use std::os::fd::AsRawFd;
use strum_macros::FromRepr;

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Default, Debug)]
pub struct Sustainer {
    pub min_percent: i8,
    pub max_percent: i8,
}

#[derive(Debug)]
pub enum SetChargeControl {
    Normal(Option<Sustainer>),
    Idle,
    Discharge,
}

pub fn supports_get_and_sustainer<File: AsRawFd>(file: &mut File) -> EcCmdResult<bool> {
    let versions = ec_cmd_get_cmd_versions(file, CrosEcCmd::ChargeControl)?;
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

impl EcParamsChargeControl {
    /// Get params to just get the charge control
    fn get() -> Self {
        let mut params = Self::zeroed();
        params.command = ChargeControlCommand::Get as u8;
        params
    }
}

impl SetChargeControl {
    fn to_set_params(&self) -> EcParamsChargeControl {
        EcParamsChargeControl {
            command: ChargeControlCommand::Set as u8,
            mode: match self {
                SetChargeControl::Normal(_) => ChargeControlMode::Normal,
                SetChargeControl::Idle => ChargeControlMode::Idle,
                SetChargeControl::Discharge => ChargeControlMode::Discharge,
            } as u32,
            reserved: Default::default(),
            sustain: match self {
                SetChargeControl::Normal(sustain) => sustain.unwrap_or(Sustainer {
                    min_percent: -1,
                    max_percent: -1,
                }),
                _ => Default::default(),
            },
        }
    }
}

#[repr(C, align(4))]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct EcResponseChargeControl {
    mode: u32,
    sustainer: Sustainer,
    reserved: u16,
}

#[repr(u8)]
#[derive(FromRepr)]
enum ChargeControlCommand {
    Set,
    Get,
}

#[repr(u32)]
#[derive(FromRepr, Debug, Clone, Copy)]
pub enum ChargeControlMode {
    Normal,
    Idle,
    Discharge,
}

#[derive(Debug, Clone, Copy)]
pub struct ChargeControlStatus {
    pub mode: ChargeControlMode,
    pub sustainer: Option<Sustainer>,
}

impl TryFrom<EcResponseChargeControl> for ChargeControlStatus {
    type Error = String;

    fn try_from(value: EcResponseChargeControl) -> Result<Self, Self::Error> {
        Ok(Self {
            mode: {
                let charge_control_mode = value.mode;
                ChargeControlMode::from_repr(charge_control_mode).ok_or(format!(
                    "Invalid charge control mode: {charge_control_mode}"
                ))?
            },
            sustainer: {
                let sustainer = value.sustainer;
                match sustainer {
                    Sustainer {
                        min_percent: -1,
                        max_percent: -1,
                    } => Ok(None),
                    Sustainer {
                        min_percent: 0..=100,
                        max_percent: 0..=100,
                    } => Ok(Some(sustainer)),
                    sustainer => Err(format!("Invalid sustainer value: {sustainer:?}")),
                }
            }?,
        })
    }
}

/// Not all Chromebooks support this. You can check if it's supported using [`supports_get_and_sustainer`]
pub fn get_charge_control<File: AsRawFd>(file: &mut File) -> EcCmdResult<ChargeControlStatus> {
    let charge_control: EcResponseChargeControl = ec_command_bytemuck(
        CrosEcCmd::ChargeControl,
        2,
        &EcParamsChargeControl::get(),
        file.as_raw_fd(),
    )?;
    Ok(charge_control.try_into().unwrap())
}

pub fn set_charge_control<File: AsRawFd>(
    file: &mut File,
    charge_control: SetChargeControl,
) -> EcCmdResult<()> {
    ec_command_bytemuck(
        CrosEcCmd::ChargeControl,
        {
            Ok(if supports_get_and_sustainer(file)? {
                2
            } else {
                1
            })
        }?,
        &charge_control.to_set_params(),
        file.as_raw_fd(),
    )?;
    Ok(())
}
