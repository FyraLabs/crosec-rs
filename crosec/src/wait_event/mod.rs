use std::io;
use std::os::fd::AsRawFd;

use nix::{
    libc::{ioctl, poll, pollfd},
    request_code_none,
};

use event::{EcMkbpEvent, EcMkbpEventType};

use crate::CROS_EC_IOC_MAGIC;

pub mod event;
pub mod fingerprint;
pub mod host_event;

const POLL_IN: i16 = 0x001;

#[derive(Debug)]
pub enum PollData {
    EventHappened(EcMkbpEvent),
    Timeout,
    SomethingElseHappened(i16),
}

/// If no timeout is specified, this function will wait for an unlimited amount of time
pub fn wait_event_sync<File: AsRawFd + std::io::Read>(
    file: &mut File,
    event_type: EcMkbpEventType,
    timeout: Option<i32>,
) -> Result<PollData, i32> {
    let mask = 1 << event_type as u8;
    unsafe {
        ioctl(
            file.as_raw_fd(),
            request_code_none!(CROS_EC_IOC_MAGIC, 2),
            mask,
        )
    };
    match timeout {
        Some(timeout) => {
            let mut fds = pollfd {
                fd: file.as_raw_fd(),
                events: POLL_IN,
                revents: Default::default(),
            };
            let result = unsafe { poll(&mut fds, 1, timeout) };
            match result {
                0 => Ok(PollData::Timeout),
                1 => match fds.revents {
                    POLL_IN => Ok(PollData::EventHappened(event_type.read_sync(file).unwrap())),
                    events => Ok(PollData::SomethingElseHappened(events)),
                },
                result => Err(result),
            }
        }
        None => Ok(PollData::EventHappened(event_type.read_sync(file).unwrap())),
    }
}

pub async fn wait_event_async<File: AsRawFd + async_std::io::Read + Unpin>(
    file: &mut File,
    event_type: EcMkbpEventType,
) -> io::Result<EcMkbpEvent> {
    let mask = 1 << event_type as u8;
    unsafe {
        ioctl(
            file.as_raw_fd(),
            request_code_none!(CROS_EC_IOC_MAGIC, 2),
            mask,
        )
    };
    event_type.read_async(file).await
}