use std::os::fd::AsRawFd;

use bytemuck::{Pod, Zeroable};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

use crate::{ec_command::ec_command_bytemuck, EcCmdResult};

use super::CrosEcCmd;

/// Note that with the ChromiumOS ectool, to start enrolling, as well as continue the next step in enrolling, you do `ectool --name=cros_fp fpmode enroll`. The equivalent of this is to do `ectool fp-mode EnrollImage EnrollSession`.
#[derive(EnumString, EnumIter, IntoStaticStr, Clone, Copy, Debug)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[repr(u32)]
pub enum FpMode {
    Reset = 0b00000000000000000000000000000000,
    DeepSleep = 0b00000000000000000000000000000001,
    FingerDown = 0b00000000000000000000000000000010,
    FingerUp = 0b00000000000000000000000000000100,
    Capture = 0b00000000000000000000000000001000,
    EnrollSession = 0b00000000000000000000000000010000,
    EnrollImage = 0b00000000000000000000000000100000,
    Match = 0b00000000000000000000000001000000,
    ResetSensor = 0b00000000000000000000000010000000,
    Maintanence = 0b00000000000000000000000100000000,
    DontChange = 0b10000000000000000000000000000000,
}

impl FpMode {
    pub fn display(fp_mode: u32) -> String {
        let flags = match fp_mode {
            0 => <FpMode as Into<&'static str>>::into(Self::Reset).to_owned(),
            fp_mode => Self::iter()
                .filter(|flag| fp_mode & *flag as u32 != 0)
                .map(<FpMode as Into<&'static str>>::into)
                .collect::<Vec<_>>()
                .join(", "),
        };
        format!("{fp_mode:#b} ({flags})")
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct EcParamsFpMode {
    mode: u32,
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct EcResponseFpMode {
    mode: u32,
}

pub fn fp_mode<File: AsRawFd>(file: &mut File, mode: u32) -> EcCmdResult<u32> {
    let response: EcResponseFpMode = ec_command_bytemuck(
        CrosEcCmd::FpMode,
        0,
        &EcParamsFpMode { mode },
        file.as_raw_fd(),
    )?;
    Ok(response.mode)
}
