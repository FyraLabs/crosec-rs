use std::ffi::c_int;

use bytemuck::{Pod, Zeroable};

use crate::{ec_command::ec_command_bytemuck, EcCmdResult};

use super::CrosEcCmd;

#[repr(C, packed)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct EcResponseFpStats {
    pub capture_time_us: u32,
    pub matching_time_us: u32,
    pub overall_time_us: u32,
    pub overall_t0: OverallT0,
    pub timestamps_invalid: u8,
    pub template_matched: i8,
}

#[repr(C, packed)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct OverallT0 {
    pub lo: u32,
    pub hi: u32,
}

pub fn fp_stats(fd: c_int) -> EcCmdResult<EcResponseFpStats> {
    ec_command_bytemuck(CrosEcCmd::FpStats, 0, &(), fd)
}
