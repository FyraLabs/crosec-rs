pub enum CrosEcCmds {
    Hello = 0x0001,
    Version = 0x0002,
    GetBuildInfo = 0x0004,
    GetChipInfo = 0x0005,
}

pub mod hello;
pub mod version;
pub mod get_chip_info;
