use std::ffi::c_int;
use std::fmt::{Debug, Display, Formatter};
use crate::commands::get_features::{ec_cmd_get_features, EC_FEATURE_PWM_FAN};
use crate::{EC_FAN_SPEED_ENTRIES, EC_FAN_SPEED_NOT_PRESENT, EC_MEM_MAP_FAN, EcError};
use crate::read_mem_any::read_mem_any;

#[derive(Debug)]
pub enum Error {
    GetFeatures(EcError),
    ReadMem(c_int),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetFeatures(e) => {
                write!(f, "Error getting features: {:#?}", e)
            },
            Self::ReadMem(e) => {
                write!(f, "Error reading memory: {:#?}", e)
            }
        }
    }
}

pub fn get_number_of_fans(fd: c_int) -> Result<usize, Error> {
    let features = ec_cmd_get_features(fd).map_err(|e| Error::GetFeatures(e))?;
    let number_of_fans = if features & EC_FEATURE_PWM_FAN != 0 {
        read_mem_any::<[u16; EC_FAN_SPEED_ENTRIES]>(fd, EC_MEM_MAP_FAN).map_err(|e| Error::ReadMem(e))?
            .into_iter()
            .filter(|data| *data != EC_FAN_SPEED_NOT_PRESENT)
            .count()
    } else {
        0
    };
    Ok(number_of_fans)
}