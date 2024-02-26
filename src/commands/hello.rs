use crate::commands::CrosEcCmds;
use crate::crosec::dev::ec_command;
use std::mem::size_of;
use std::slice;

const INPUT_DATA: u32 = 0xa0b0c0d0;
const EXPECTED_OUTPUT: u32 = 0xa1b2c3d4;

#[repr(C, align(4))]
struct EcParamsHello {
    in_data: u32,
}

#[repr(C, align(4))]
struct EcResponseHello {
    out_data: u32,
}

pub fn ec_cmd_hello() -> bool {
    let params = EcParamsHello {
        in_data: INPUT_DATA,
    };
    let params_ptr = &params as *const _ as *const u8;
    let params_slice = unsafe { slice::from_raw_parts(params_ptr, size_of::<EcParamsHello>()) };

    let result = ec_command(CrosEcCmds::Hello as u32, 0, params_slice)
        .unwrap_or_else(|error| panic!("EC error: {error:?}"));
    let response: EcResponseHello = unsafe { std::ptr::read(result.as_ptr() as *const _) };
    response.out_data == EXPECTED_OUTPUT
}
