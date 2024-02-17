use crate::commands::CrosEcCmds;
use crate::crosec::dev::ec_command;
use std::slice;
use std::mem::size_of;

const INPUT_DATA: u32 = 0xa0b0c0d0;
const EXPECTED_OUTPUT: u32 = 0xa1b2c3d4;

#[repr(C, align(4))]
struct ec_params_hello {
    in_data: u32,
}

#[repr(C, align(4))]
struct ec_response_hello {
    out_data: u32,
}

pub fn ec_cmd_hello() {
    let params = ec_params_hello {
        in_data: INPUT_DATA,
    };
    let params_ptr = &params as *const _ as *const u8;
    let params_slice = unsafe { slice::from_raw_parts(params_ptr, size_of::<ec_params_hello>()) };

    let result = ec_command(CrosEcCmds::Hello as u32, 0, params_slice);
    let result_data = match result {
        Ok(data) => data,
        Err(error) => panic!("EC error: {:?}", error),
    };
    let response: ec_response_hello = unsafe { std::ptr::read(result_data.as_ptr() as *const _) };
    if response.out_data == EXPECTED_OUTPUT {
        println!("Ec says hello!");
    }
}