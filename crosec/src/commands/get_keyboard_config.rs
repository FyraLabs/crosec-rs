use super::CrosEcCmd;
use crate::{ec_command::ec_command_bytemuck, EcCmdResult};
use bytemuck::{Pod, Zeroable};
use num_derive::FromPrimitive;
use std::os::fd::AsRawFd;

const MAX_TOP_ROW_KEYS: usize = 15;

/// Is the keyboard capable of sending function keys *in addition to*
/// action keys. This is possible for e.g. if the keyboard has a
/// dedicated Fn key.
pub const KEYBD_CAP_FUNCTION_KEYS: u8 = 1;
/// Whether the keyboard has a dedicated numeric keyboard.
pub const KEYBD_CAP_NUMERIC_KEYPAD: u8 = 2;
/// Whether the keyboard has a screenlock key.
pub const KEYBD_CAP_SCRNLOCK_KEY: u8 = 4;
/// Whether the keyboard has an assistant key.
pub const KEYBD_CAP_ASSISTANT_KEY: u8 = 8;

#[repr(C)]
#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum ActionKey {
    TkAbsent = 0,
    TkBack = 1,
    TkForward = 2,
    TkRefresh = 3,
    TkFullscreen = 4,
    TkOverview = 5,
    TkBrightnessDown = 6,
    TkBrightnessUp = 7,
    TkVolMute = 8,
    TkVolDown = 9,
    TkVolUp = 10,
    TkSnapshot = 11,
    TkPrivacyScrnToggle = 12,
    TkKbdBklightDown = 13,
    TkKbdBklightUp = 14,
    TkPlayPause = 15,
    TkNextTrack = 16,
    TkPrevTrack = 17,
    TkKbdBklightToggle = 18,
    TkMicmute = 19,
    TkMenu = 20,
    TkDictate = 21,
    TkAccessibility = 22,
    TkDonotdisturb = 23,
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct EcResponseKeybdConfig {
    /// Number of top row keys, excluding Esc and Screenlock.
    /// If this is 0, all Vivaldi keyboard code is disabled.
    /// (i.e. does not expose any tables to the kernel).
    pub num_top_row_keys: u8,

    /// Empty log entries have both the cause and timestamp set to zero.
    pub action_keys: [u8; MAX_TOP_ROW_KEYS],

    /// Capability flags
    pub capabilities: u8,
}

pub fn ec_cmd_get_keyboard_config<File: AsRawFd>(
    file: &mut File,
) -> EcCmdResult<EcResponseKeybdConfig> {
    ec_command_bytemuck(CrosEcCmd::GetKeybdConfig, 0, &(), file.as_raw_fd())
}
