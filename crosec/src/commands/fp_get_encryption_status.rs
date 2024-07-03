use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command_bytemuck;
use crate::EcCmdResult;
use bytemuck::{Pod, Zeroable};
use std::os::fd::AsRawFd;

#[repr(u32)]
pub enum FpEncryptionStatus {
    SeedSet = 0b1,
}

#[derive(Pod, Zeroable, Copy, Clone)]
#[repr(C)]
pub struct EcResponseFpGetEncryptionStatus {
    pub valid_flags: u32,
    pub status: u32,
}

pub fn fp_get_encryption_status<File: AsRawFd>(
    file: &mut File,
) -> EcCmdResult<EcResponseFpGetEncryptionStatus> {
    ec_command_bytemuck(CrosEcCmd::FpGetEncryptionStatus, 0, &(), file.as_raw_fd())
}
