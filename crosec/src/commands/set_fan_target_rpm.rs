use std::fs::File;
use std::os::fd::AsRawFd;

use bytemuck::{NoUninit, Pod, Zeroable};

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command_bytemuck;
use crate::EcCmdResult;

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
struct EcParamsSetFanTargetRpmV0 {
    rpm: u32,
}

#[derive(Clone, Copy, NoUninit)]
#[repr(C, align(1))]
struct EcParamsSetFanTargetRpmV1 {
    rpm: u32,
    fan_index: u8,
    _padding: [u8; 3],
}

// impl EcParamsSetFanTargetRpmV1 {
//     pub fn to_le_bytes(self) -> [u8; size_of::<u32>() + size_of::<u8>()] {
//         [
//             self.rpm.to_le_bytes().to_vec(),
//             self.fan_index.to_le_bytes().to_vec(),
//         ]
//         .concat()
//         .try_into()
//         .unwrap()
//     }
// }

pub fn ec_cmd_set_fan_target_rpm(
    file: &mut File,
    rpm: u32,
    fan_index: Option<u8>,
) -> EcCmdResult<()> {
    // v0 can only set the RPM for all fans
    // v1 can set the RPM for a specific fan
    match fan_index {
        Some(index) => {
            ec_command_bytemuck(
                CrosEcCmd::SetFanTargetRpm,
                1,
                &EcParamsSetFanTargetRpmV1 {
                    rpm,
                    fan_index: index,
                    _padding: Default::default(),
                },
                file.as_raw_fd(),
            )?;
        }
        None => {
            ec_command_bytemuck(
                CrosEcCmd::SetFanTargetRpm,
                0,
                &EcParamsSetFanTargetRpmV0 { rpm },
                file.as_raw_fd(),
            )?;
        }
    };
    Ok(())
}
