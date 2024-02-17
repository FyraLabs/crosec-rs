use num_derive::FromPrimitive;
pub mod dev;

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

#[derive(Debug)]
pub enum EcError {
    Response(EcResponseStatus),
    UnknownResponseCode(u32),
    DeviceError(String),
}

pub type EcCmdResult<T> = Result<T, EcError>;