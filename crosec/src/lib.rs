use nix::errno::Errno;
use num_derive::FromPrimitive;
use thiserror::Error;

pub mod commands;
pub mod ec_command;
pub mod read_mem_any;
pub mod get_number_of_fans;

// In the future, portio should be supported as well
pub enum EcInterface {
    Dev(String),
    Default,
}

#[derive(FromPrimitive, Debug, Copy, Clone)]
pub enum EcResponseStatus {
    Success = 0,
    InvalidCommand = 1,
    Error = 2,
    InvalidParam = 3,
    AccessDenied = 4,
    InvalidResponse = 5,
    InvalidVersion = 6,
    InvalidChecksum = 7,
    InProgress = 8,
    Unavailable = 9,
    Timeout = 10,
    Overflow = 11,
    InvalidHeader = 12,
    RequestTruncated = 13,
    ResponseTooBig = 14,
    BusError = 15,
    Busy = 16,
    InvalidHeaderVersion = 17,
    InvalidHeaderCRC = 18,
    InvalidDataCRC = 19,
    DUPUnavailable = 20,
}

#[derive(Error, Debug)]
pub enum EcError {
    #[error("command failed with status {0:?}")]
    Response(EcResponseStatus),
    #[error("received unknown response code {0}")]
    UnknownResponseCode(u32),
    #[error("device error with errno {0:?}")]
    DeviceError(Errno),
}

pub type EcCmdResult<T> = Result<T, EcError>;

pub const EC_FAN_SPEED_ENTRIES: usize = 4;
pub const EC_FAN_SPEED_NOT_PRESENT: u16 = 0xffff;
pub const EC_MEM_MAP_FAN: u8 = 0x10;
pub const CROS_EC_IOC_MAGIC: u8 = 0xEC;
