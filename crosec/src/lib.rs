use nix::errno::Errno;
use num_derive::FromPrimitive;
use thiserror::Error;

pub mod battery;
pub mod commands;
pub mod console;
pub mod ec_command;
pub mod get_number_of_fans;
pub mod read_mem_any;
pub mod read_mem_string;
pub mod wait_event;

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

pub const CROS_EC_PATH: &str = "/dev/cros_ec";
pub const CROS_FP_PATH: &str = "/dev/cros_fp";

pub const EC_FAN_SPEED_ENTRIES: usize = 4;
pub const EC_FAN_SPEED_NOT_PRESENT: u16 = 0xffff;
pub const EC_FAN_SPEED_STALLED: u16 = 0xfffe;
pub const EC_MEM_MAP_MAX_TEXT_SIZE: usize = 8;

pub const EC_MEM_MAP_FAN: u8 = 0x10;
/// Version of data in 0x40 - 0x7f
pub const EC_MEM_MAP_BATTERY_VERSION: u8 = 0x24;
/// Battery Present Voltage
pub const EC_MEM_MAP_BATTERY_VOLTAGE: u8 = 0x40;
/// Battery Present Rate
pub const EC_MEM_MAP_BATTERY_RATE: u8 = 0x44;
/// Battery Remaining Capacity
pub const EC_MEM_MAP_BATTERY_CAPACITY: u8 = 0x48;
/// Battery State, see below (8-bit)
pub const EC_MEM_MAP_BATTERY_FLAGS: u8 = 0x4c;
/// Battery Count (8-bit)
pub const EC_MEM_MAP_BATTERY_COUNT: u8 = 0x4d;
/// Current Battery Data Index (8-bit)
pub const EC_MEM_MAP_BATTERY_INDEX: u8 = 0x4e;
pub const EC_MEM_MAP_BATTERY_DESIGN_CAPACITY: u8 = 0x50;
pub const EC_MEM_MAP_BATTERY_DESIGN_VOLTAGE: u8 = 0x54;
pub const EC_MEM_MAP_BATTERY_LAST_FULL_CHARGE_CAPACITY: u8 = 0x58;
pub const EC_MEM_MAP_BATTERY_CYCLE_COUNT: u8 = 0x5c;
pub const EC_MEM_MAP_BATTERY_MANUFACTURER: u8 = 0x60;
pub const EC_MEM_MAP_BATTERY_MODEL: u8 = 0x68;
pub const EC_MEM_MAP_BATTERY_SERIAL: u8 = 0x70;
pub const EC_MEM_MAP_BATTERY_TYPE: u8 = 0x78;

pub const CROS_EC_IOC_MAGIC: u8 = 0xEC;
