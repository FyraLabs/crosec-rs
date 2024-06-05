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
    FpInfo = 0x0403,
    BatteryGetStatic = 0x0600,
}

pub mod board_version;
pub mod charge_control;
pub mod fp_info;
pub mod get_chip_info;
pub mod get_cmd_versions;
pub mod get_features;
pub mod get_protocol_info;
pub mod hello;
pub mod read_mem;
pub mod set_fan_target_rpm;
pub mod version;
