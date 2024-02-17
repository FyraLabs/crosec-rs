use nix::ioctl_readwrite;
use std::os::unix::io::AsRawFd;
use num_traits::FromPrimitive;
use crate::crosec::EcCmdResult;
use crate::crosec::EcError;

use super::EcResponseStatus;

const IN_SIZE: usize = 256;

#[repr(C)]
pub struct _CrosEcCommandV2 {
    version: u32,
    command: u32,
    outsize: u32,
    insize: u32,
    result: u32,
    data: [u8; 0],
}
#[repr(C)]
struct CrosEcCommandV2 {
    version: u32,
    command: u32,
    outsize: u32,
    insize: u32,
    result: u32,
    data: [u8; IN_SIZE],
}

const DEV_PATH: &str = "/dev/cros_ec";

static mut CROS_EC_FD: Option<std::fs::File> = None;

const CROS_EC_IOC_MAGIC: u8 = 0xEC;
ioctl_readwrite!(cros_ec_cmd, CROS_EC_IOC_MAGIC, 0, _CrosEcCommandV2);

fn get_fildes() -> i32 {
    unsafe { CROS_EC_FD.as_ref().unwrap().as_raw_fd() }
}

fn init() {
    match std::fs::File::open(DEV_PATH) {
        Err(why) => println!("Failed to open {}. Because: {:?}", DEV_PATH, why),
        Ok(file) => unsafe { CROS_EC_FD = Some(file) },
    };
}

pub fn ec_command(command: u32, command_version: u8, data: &[u8]) -> EcCmdResult<Vec<u8>> {
    init();

    let size = std::cmp::min(IN_SIZE, data.len());

    let mut cmd = CrosEcCommandV2 {
        version: command_version as u32,
        command: command,
        outsize: size as u32,
        insize: IN_SIZE as u32,
        result: 0xFF,
        data: [0; IN_SIZE],
    };

    cmd.data[0..size].copy_from_slice(data);
    let cmd_ptr = &mut cmd as *mut _ as *mut _CrosEcCommandV2;


    unsafe {
        let result = cros_ec_cmd(get_fildes(), cmd_ptr);
        let status: Option<EcResponseStatus> = FromPrimitive::from_u32(cmd.result);

        match &status {
            None => return Err(EcError::UnknownResponseCode(cmd.result)),
            Some(EcResponseStatus::Success) => {},
            Some(status) => return Err(EcError::Response(*status)),
        }

        match result {
            Ok(result) => {
                let result_size = result as usize;
                let result_data = &cmd.data[0..result_size];
                Ok(result_data.to_vec())
            }
            Err(err) => Err(EcError::DeviceError(format!(
                "ioctl to send command to EC failed with {:?}", err
            )))
        }


    }
}