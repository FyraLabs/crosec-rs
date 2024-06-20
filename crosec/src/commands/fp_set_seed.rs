use std::os::fd::AsRawFd;

use bytemuck::{Pod, Zeroable};

use crate::{ec_command::ec_command_bytemuck, EcCmdResult};

use super::CrosEcCmd;

pub const FP_CONTEXT_TPM_BYTES: usize = 32;
const FP_TEMPLATE_FORMAT_VERSION: u16 = 4;

#[repr(C, align(4))]
#[derive(Pod, Zeroable, Clone, Copy)]
struct EcParamsFpSeed {
    pub struct_version: u16,
    pub reserved: u16,
    pub seed: [u8; FP_CONTEXT_TPM_BYTES],
}

pub fn fp_set_seed<File: AsRawFd>(file: &mut File, seed: [u8; FP_CONTEXT_TPM_BYTES]) -> EcCmdResult<()> {
    ec_command_bytemuck(
        CrosEcCmd::FpSetSeed,
        0,
        &EcParamsFpSeed {
            struct_version: FP_TEMPLATE_FORMAT_VERSION,
            reserved: Default::default(),
            seed,
        },
        file.as_raw_fd(),
    )
}
