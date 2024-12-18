use std::{os::fd::AsRawFd, thread::sleep, time::Duration};

use bytemuck::{Pod, Zeroable};
use num::ToPrimitive;
use num_derive::ToPrimitive;

use crate::{ec_command::ec_command_bytemuck, EcCmdResult};

use super::CrosEcCmd;

pub type UserId = [u8; 32];

/* Version 1 of the command is "asynchronous". */
#[repr(C, align(4))]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
struct EcParamsFpContextV1 {
    /**< enum fp_context_action */
    action: u8,
    reserved: [u8; 3],
    user_id: UserId,
}

#[repr(u8)]
#[derive(ToPrimitive)]
enum FpContextAction {
    Async = 0,
    GetResult = 1,
}

/// Make sure that the fp mode is Reset before setting the context
/// Related: https://chromium.googlesource.com/chromiumos/platform2/+/HEAD/biod/cros_fp_device.cc#660
pub fn fp_set_context<File: AsRawFd>(file: &mut File, user_id: UserId) -> EcCmdResult<()> {
    // From testing, it seems that this can be anything besides all zeroes, but we're going to use these numbers in honor of CoolStar - https://github.com/coolstar/crosfingerprint/blob/5e77307d7542218e173f24eb657b426565ed361a/fingerprint_adapter/eccmd.cpp#L140
    ec_command_bytemuck(
        CrosEcCmd::FpContext,
        1,
        &EcParamsFpContextV1 {
            action: FpContextAction::Async.to_u8().unwrap(),
            reserved: Default::default(),
            user_id,
        },
        file.as_raw_fd(),
    )?;
    let mut tries = 20;
    let delay = Duration::from_millis(100);
    loop {
        sleep(delay);
        let result = ec_command_bytemuck(
            CrosEcCmd::FpContext,
            1,
            &EcParamsFpContextV1 {
                action: FpContextAction::GetResult.to_u8().unwrap(),
                reserved: Default::default(),
                user_id,
            },
            file.as_raw_fd(),
        );
        if result.is_ok() {
            break result;
        } else {
            tries -= 1;
            if tries == 0 {
                break result;
            }
        }
    }
}
