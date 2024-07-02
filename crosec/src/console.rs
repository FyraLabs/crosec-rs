use std::fs::File;
use std::os::fd::AsRawFd;

use crate::commands::get_protocol_info::EcResponseGetProtocolInfo;
use crate::commands::CrosEcCmd;
use crate::ec_command::{ec_command_bytemuck, ec_command_with_dynamic_output_size};
use crate::EcCmdResult;

pub fn console(file: &mut File, protocol_info: &EcResponseGetProtocolInfo) -> EcCmdResult<String> {
    ec_command_bytemuck(CrosEcCmd::ConsoleSnapshot, 0, &(), file.as_raw_fd())?;
    let mut console = String::default();
    loop {
        let output = ec_command_with_dynamic_output_size(
            CrosEcCmd::ConsoleRead,
            0,
            Default::default(),
            protocol_info.max_ec_output_size(),
            file.as_raw_fd(),
        )?;
        let chunk = String::from_utf8(output).unwrap();
        // Get rid of trailing null characters
        let chunk = chunk.trim_end_matches('\0');
        if !chunk.is_empty() {
            console += &chunk;
        } else {
            break;
        }
    }
    Ok(console)
}
