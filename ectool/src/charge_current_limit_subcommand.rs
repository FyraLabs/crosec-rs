use std::fs::File;

use color_eyre::eyre::Result;
use crosec::{commands::charge_current_limit::set_charge_current_limit, CROS_EC_PATH};
use uom::si::electric_current::{milliampere, ElectricCurrent};

pub fn charge_current_limit_subcommand(limit: u32) -> Result<()> {
    let mut file = File::open(CROS_EC_PATH)?;
    set_charge_current_limit(&mut file, ElectricCurrent::new::<milliampere>(limit as f32))?;
    Ok(())
}
