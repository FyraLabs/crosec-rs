use bytemuck::{NoUninit, Pod, Zeroable};

use crate::{commands::CrosEcCmd, ec_command, EcCmdResult, EcInterface};

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

pub fn ec_cmd_hello() -> EcCmdResult<bool> {
    let params = EcParamsHello {
        in_data: INPUT_DATA,
    };
    let params_slice = bytemuck::bytes_of(&params);

    let result = ec_command(CrosEcCmd::Hello, 0, params_slice, EcInterface::Dev(String::from("/dev/cros_ec")))?;
    Ok(bytemuck::try_from_bytes::<EcResponseHello>(&result).map_or(
        false,
        |response| response.out_data == EXPECTED_OUTPUT)
    )
}
