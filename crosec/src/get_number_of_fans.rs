use crate::commands::get_features::{ec_cmd_get_features, EC_FEATURE_PWM_FAN};
use crate::read_mem_any::read_mem_any;
use crate::{EcError, EC_FAN_SPEED_ENTRIES, EC_FAN_SPEED_NOT_PRESENT, EC_MEM_MAP_FAN};
use std::ffi::c_int;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;

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
            }
            Self::ReadMem(e) => {
                write!(f, "Error reading memory: {:#?}", e)
            }
        }
    }
}

pub fn get_number_of_fans(file: &mut File) -> Result<usize, Error> {
    let features = ec_cmd_get_features(file).map_err(Error::GetFeatures)?;
    let number_of_fans = if features & EC_FEATURE_PWM_FAN != 0 {
        read_mem_any::<[u16; EC_FAN_SPEED_ENTRIES]>(file, EC_MEM_MAP_FAN)
            .map_err(Error::ReadMem)?
            .into_iter()
            .filter(|data| *data != EC_FAN_SPEED_NOT_PRESENT)
            .count()
    } else {
        0
    };
    Ok(number_of_fans)
}
