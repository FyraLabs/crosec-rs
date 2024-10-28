use std::os::fd::AsRawFd;

use bytemuck::{Pod, Zeroable};

use crate::{ec_command::ec_command_bytemuck, EcCmdResult};

use super::CrosEcCmd;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct EcResponseUptimeInfo {
    /// Number of milliseconds since the last EC boot. Sysjump resets
    /// typically do not restart the EC's time_since_boot epoch.
    ///
    /// WARNING: The EC's sense of time is much less accurate than the AP's
    /// sense of time, in both phase and frequency. This timebase is similar
    /// to CLOCK_MONOTONIC_RAW, but with 1% or more frequency error.
    pub time_since_ec_boot_ms: u32,

    /// Number of times the AP was reset by the EC since the last EC boot.
    /// Note that the AP may be held in reset by the EC during the initial
    /// boot sequence, such that the very first AP boot may count as more
    /// than one here.
    pub ap_resets_since_ec_boot: u32,

    /// The set of flags which describe the EC's most recent reset.
    /// See EC_RESET_FLAG_* for details.
    pub ec_reset_flags: u32,

    /// Empty log entries have both the cause and timestamp set to zero.
    pub recent_ap_reset: [ApResetLogEntry; 4],
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct ApResetLogEntry {
    /// See enum chipset_{reset,shutdown}_reason for details.
    pub reset_cause: u16,

    /// Reserved for protocol growth.
    pub reserved: u16,

    /// The time of the reset's assertion, in milliseconds since the
    /// last EC boot, in the same epoch as time_since_ec_boot_ms.
    /// Set to zero if the log entry is empty.
    pub reset_time_ms: u32,
}

pub fn ec_cmd_get_uptime_info<File: AsRawFd>(file: &mut File) -> EcCmdResult<EcResponseUptimeInfo> {
    ec_command_bytemuck(CrosEcCmd::GetUptimeInfo, 0, &(), file.as_raw_fd())
}
