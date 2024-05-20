use std::ffi::c_int;
use crate::commands::CrosEcCmd;
use crate::commands::get_cmd_versions::{ec_cmd_get_cmd_versions, V1};
use crate::{EC_MEM_MAP_BATTERY_CAPACITY, EC_MEM_MAP_BATTERY_CYCLE_COUNT, EC_MEM_MAP_BATTERY_DESIGN_CAPACITY, EC_MEM_MAP_BATTERY_DESIGN_VOLTAGE, EC_MEM_MAP_BATTERY_FLAGS, EC_MEM_MAP_BATTERY_LAST_FULL_CHARGE_CAPACITY, EC_MEM_MAP_BATTERY_MANUFACTURER, EC_MEM_MAP_BATTERY_MODEL, EC_MEM_MAP_BATTERY_RATE, EC_MEM_MAP_BATTERY_SERIAL, EC_MEM_MAP_BATTERY_TYPE, EC_MEM_MAP_BATTERY_VERSION, EC_MEM_MAP_BATTERY_VOLTAGE, EcCmdResult};
use crate::read_mem_any::read_mem_any;
use crate::read_mem_string::read_mem_string;

#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub oem_name: String,
    pub model_number: String,
    pub chemistry: String,
    pub serial_number: String,
    pub design_capacity: i32,
    pub last_full_charge: i32,
    pub design_output_voltage: i32,
    pub cycle_count: i32,
    pub present_voltage: i32,
    pub present_current: i32,
    pub remaining_capacity: i32,
    pub flags: u8,
}

pub fn battery(fd: c_int) -> EcCmdResult<BatteryInfo> {
    if ec_cmd_get_cmd_versions(fd, CrosEcCmd::BatteryGetStatic)? & V1 != 0 {
        panic!("Battery info needs to be gotten with the {:?} command", CrosEcCmd::BatteryGetStatic);
    } else {
        let battery_version = read_mem_any::<i8>(fd, EC_MEM_MAP_BATTERY_VERSION).unwrap();
        if battery_version < 1 {
            panic!("Battery version {battery_version} is not supported");
        }
        let flags = read_mem_any::<u8>(fd, EC_MEM_MAP_BATTERY_FLAGS).unwrap();
        let oem_name = read_mem_string(fd, EC_MEM_MAP_BATTERY_MANUFACTURER).unwrap();
        let model_number = read_mem_string(fd, EC_MEM_MAP_BATTERY_MODEL).unwrap();
        let chemistry = read_mem_string(fd, EC_MEM_MAP_BATTERY_TYPE).unwrap();
        let serial_number = read_mem_string(fd, EC_MEM_MAP_BATTERY_SERIAL).unwrap();
        let design_capacity = read_mem_any::<i32>(fd, EC_MEM_MAP_BATTERY_DESIGN_CAPACITY).unwrap();
        let last_full_charge = read_mem_any::<i32>(fd, EC_MEM_MAP_BATTERY_LAST_FULL_CHARGE_CAPACITY).unwrap();
        let design_output_voltage = read_mem_any::<i32>(fd, EC_MEM_MAP_BATTERY_DESIGN_VOLTAGE).unwrap();
        let cycle_count = read_mem_any::<i32>(fd, EC_MEM_MAP_BATTERY_CYCLE_COUNT).unwrap();
        let present_voltage = read_mem_any::<i32>(fd, EC_MEM_MAP_BATTERY_VOLTAGE).unwrap();
        let present_current = read_mem_any::<i32>(fd, EC_MEM_MAP_BATTERY_RATE).unwrap();
        let remaining_capacity = read_mem_any::<i32>(fd, EC_MEM_MAP_BATTERY_CAPACITY).unwrap();
        Ok(BatteryInfo {
            flags,
            oem_name,
            model_number,
            chemistry,
            serial_number,
            design_capacity,
            last_full_charge,
            design_output_voltage,
            cycle_count,
            present_voltage,
            present_current,
            remaining_capacity
        })
    }
}