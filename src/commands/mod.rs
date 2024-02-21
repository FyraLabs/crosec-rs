pub enum CrosEcCmds {
    Hello = 0x0001,
    Version = 0x0002,

    GetBuildInfo = 0x0004,
}

pub mod hello;
pub mod version;
