use num_derive::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive, Debug)]
#[repr(u32)]
pub enum CrosEcCmd {
    Hello = 0x0001,
    Version = 0x0002,
    GetBuildInfo = 0x0004,
    GetChipInfo = 0x0005,
    GetBoardVersion = 0x0006,
    ReadMemMap = 0x0007,
    GetCmdVersions = 0x0008,
    GetProtocolInfo = 0x000B,
    GetFeatures = 0x000D,
    SetFanTargetRpm = 0x0021,
    ChargeControl = 0x0096,
    ConsoleSnapshot = 0x0097,
    ConsoleRead = 0x0098,
    GetUptimeInfo = 0x0121,
    GetKeybdConfig = 0x012A,
    FpMode = 0x0402,
    FpInfo = 0x0403,
    FpFrame = 0x0404,
    FpTemplate = 0x0405,
    FpStats = 0x0407,
    FpSetSeed = 0x0408,
    FpGetEncryptionStatus = 0x0409,
    BatteryGetStatic = 0x0600,
    ChargeCurrentLimit = 0x00A1,
}

pub mod board_version;
pub mod charge_control;
pub mod charge_current_limit;
pub mod fp_download;
pub mod fp_get_encryption_status;
pub mod fp_info;
pub mod fp_mode;
pub mod fp_set_seed;
pub mod fp_stats;
pub mod fp_upload_template;
pub mod get_chip_info;
pub mod get_cmd_versions;
pub mod get_features;
pub mod get_keyboard_config;
pub mod get_protocol_info;
pub mod get_uptime_info;
pub mod hello;
pub mod read_mem;
pub mod set_fan_target_rpm;
pub mod version;
