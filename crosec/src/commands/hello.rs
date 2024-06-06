use std::fs::File;
use std::os::fd::AsRawFd;

use bytemuck::{NoUninit, Pod, Zeroable};

use crate::ec_command::ec_command_bytemuck;
use crate::{commands::CrosEcCmd, EcCmdResult};

const INPUT_DATA: u32 = 0xa0b0c0d0;
const EXPECTED_OUTPUT: u32 = 0xa1b2c3d4;

#[repr(C, align(4))]
#[derive(NoUninit, Copy, Clone)]
struct EcParamsHello {
    in_data: u32,
}

#[repr(C, align(4))]
#[derive(Pod, Zeroable, Copy, Clone)]
struct EcResponseHello {
    out_data: u32,
}

pub fn ec_cmd_hello(file: &mut File) -> EcCmdResult<bool> {
    let response = ec_command_bytemuck::<_, EcResponseHello>(
        CrosEcCmd::Hello,
        0,
        &EcParamsHello {
            in_data: INPUT_DATA,
        },
        file.as_raw_fd(),
    )?;
    Ok(response.out_data == EXPECTED_OUTPUT)
}
