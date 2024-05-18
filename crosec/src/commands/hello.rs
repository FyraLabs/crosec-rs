use bytemuck::{NoUninit, Pod, Zeroable};
use nix::libc::c_int;

use crate::{commands::CrosEcCmd, EcCmdResult};
use crate::ec_command::ec_command;

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

pub fn ec_cmd_hello(fd: c_int) -> EcCmdResult<bool> {
    let params = EcParamsHello {
        in_data: INPUT_DATA,
    };
    let params_slice = bytemuck::bytes_of(&params);

    let result = ec_command(CrosEcCmd::Hello, 0, params_slice, fd)?;
    Ok(bytemuck::try_from_bytes::<EcResponseHello>(&result).map_or(
        false,
        |response| response.out_data == EXPECTED_OUTPUT)
    )
}
