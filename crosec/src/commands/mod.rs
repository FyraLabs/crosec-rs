use num_derive::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive)]
#[repr(u32)]
pub enum CrosEcCmd {
    Hello = 0x0001,
    Version = 0x0002,
    GetBuildInfo = 0x0004,
    GetChipInfo = 0x0005,
    GetBoardVersion = 0x0006,
    GetCmdVersions = 0x0008,
}

pub mod get_chip_info;
pub mod hello;
pub mod version;
pub mod board_version;
pub mod get_cmd_versions;
