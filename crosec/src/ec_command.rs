use crate::commands::CrosEcCmd;
use crate::EcError;
use crate::{EcCmdResult, CROS_EC_IOC_MAGIC};
use bytemuck::{bytes_of, from_bytes, AnyBitPattern, NoUninit, Pod, Zeroable};
use nix::ioctl_readwrite;
use num_traits::FromPrimitive;
use std::cmp::max;
use std::mem::size_of;
use std::os::raw::c_int;

use super::EcResponseStatus;

#[derive(Debug, Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct CrosEcCommandV2 {
    version: u32,
    command: u32,
    ec_input_size: u32,
    ec_output_size: u32,
    result: u32,
    data: [u8; 0],
}

ioctl_readwrite!(cros_ec_cmd, CROS_EC_IOC_MAGIC, 0, CrosEcCommandV2);

pub fn ec_command_with_dynamic_output_size(
    command: CrosEcCmd,
    command_version: u8,
    input_buffer: &[u8],
    output_size: usize,
    fd: c_int,
) -> EcCmdResult<Vec<u8>> {
    let buffer_size = max(input_buffer.len(), output_size);
    let cmd_without_data = CrosEcCommandV2 {
        version: command_version as u32,
        command: command as u32,
        ec_input_size: input_buffer.len() as u32,
        ec_output_size: output_size as u32,
        result: 0xFF,
        data: [],
    };
    let mut cmd_vec = bytemuck::bytes_of(&cmd_without_data).to_vec();
    cmd_vec.extend({
        let mut buffer = input_buffer.to_vec();
        buffer.resize(buffer_size, Default::default());
        buffer
    });
    let result = unsafe { cros_ec_cmd(fd, cmd_vec.as_mut_ptr() as *mut _ as *mut CrosEcCommandV2) };
    let _output_size = result.map_err(EcError::DeviceError)?;
    let cmd_without_data =
        bytemuck::from_bytes::<CrosEcCommandV2>(&cmd_vec[..size_of::<CrosEcCommandV2>()]);
    let status = FromPrimitive::from_u32(cmd_without_data.result)
        .ok_or(EcError::UnknownResponseCode(cmd_without_data.result))?;
    match status {
        EcResponseStatus::Success => Ok(cmd_vec
            [size_of::<CrosEcCommandV2>()..size_of::<CrosEcCommandV2>() + output_size]
            .to_vec()),
        status => Err(EcError::Response(status)),
    }
}

pub fn ec_command_bytemuck<Request: NoUninit, Response: AnyBitPattern>(
    command: CrosEcCmd,
    command_version: u8,
    input: &Request,
    fd: c_int,
) -> EcCmdResult<Response> {
    let response = ec_command_with_dynamic_output_size(
        command,
        command_version,
        bytes_of(input),
        size_of::<Response>(),
        fd,
    )?;
    Ok(from_bytes::<Response>(&response).to_owned())
}
