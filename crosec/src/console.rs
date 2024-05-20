use std::ffi::c_int;

use crate::commands::CrosEcCmd;
use crate::ec_command::ec_command;
use crate::EcCmdResult;

pub fn console(fd: c_int) -> EcCmdResult<String> {
    ec_command(CrosEcCmd::ConsoleSnapshot, 0, Default::default(), fd)?;
    let mut console = String::default();
    loop {
        let output = ec_command(CrosEcCmd::ConsoleRead, 0, Default::default(), fd)?;
        let chunk = String::from_utf8(output).unwrap();
        if !chunk.is_empty() {
            console += &chunk;
        } else {
            break;
        }
    }
    Ok(console)
}
