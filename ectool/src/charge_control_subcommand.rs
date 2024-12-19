use std::fs::File;

use clap::Subcommand;
use color_eyre::eyre::Result;
use crosec::{
    commands::charge_control::{
        get_charge_control, set_charge_control, supports_get_and_sustainer, SetChargeControl,
        Sustainer,
    },
    CROS_EC_PATH,
};

#[derive(Subcommand)]
pub enum ChargeControlSubcommand {
    /// Charge the battery with external power and power the device with external power
    Normal {
        /// Minimum battery % to keep the battery at
        min_percent: Option<u8>,
        /// Maximum battery & to keep the battery at. If this isn't specified, this will be set to the same as the min %.
        max_percent: Option<u8>,
    },
    /// Power the device with external power, but do not charge the battery
    Idle,
    /// Power the device with the battery and do not charge the battery
    Discharge,
}

pub fn charge_control_subcommand(command: Option<ChargeControlSubcommand>) -> Result<()> {
    {
        let mut file = File::open(CROS_EC_PATH)?;
        match command {
            None => {
                if supports_get_and_sustainer(&mut file)? {
                    let charge_control = get_charge_control(&mut file)?;
                    println!("{charge_control:#?}");
                } else {
                    println!("This EC doesn't support getting charge control");
                }
            }
            Some(command) => match command {
                ChargeControlSubcommand::Normal {
                    min_percent,
                    max_percent,
                } => match min_percent {
                    Some(min_percent) => {
                        let max_percent = max_percent.unwrap_or(min_percent);
                        set_charge_control(
                            &mut file,
                            SetChargeControl::Normal(Some(Sustainer {
                                min_percent: min_percent as i8,
                                max_percent: max_percent as i8,
                            })),
                        )?;
                        println!("Set charge control to normal with sustainer from {min_percent}% to {max_percent}%");
                    }
                    None => {
                        set_charge_control(&mut file, SetChargeControl::Normal(None))?;
                        println!("Set charge control to normal");
                    }
                },
                ChargeControlSubcommand::Idle => {
                    println!("Set charge control to idle");
                    set_charge_control(&mut file, SetChargeControl::Idle)?;
                }
                ChargeControlSubcommand::Discharge => {
                    println!("Set charge control to discharge");
                    set_charge_control(&mut file, SetChargeControl::Discharge)?;
                }
            },
        }
    }
    Ok(())
}
