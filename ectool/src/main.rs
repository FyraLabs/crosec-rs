use std::fs::File;
use std::os::fd::AsRawFd;
use std::str::FromStr;

use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use crosec::commands::fp_info::fp_info;
use crosec::commands::fp_mode::{fp_mode, FpMode};
use crosec::commands::fp_set_seed::{fp_set_seed, FP_CONTEXT_TPM_BYTES};
use crosec::commands::fp_stats::fp_stats;
use crosec::commands::get_protocol_info::get_protocol_info;
use crosec::wait_event::{event::EcMkbpEventType, wait_event};
use num_traits::cast::FromPrimitive;

use crosec::battery::battery;
use crosec::commands::board_version::ec_cmd_board_version;
use crosec::commands::charge_control::{
    get_charge_control, set_charge_control, supports_get_and_sustainer, Sustainer,
};
use crosec::commands::get_cmd_versions::ec_cmd_get_cmd_versions;
use crosec::commands::get_features::{ec_cmd_get_features, EC_FEATURE_PWM_FAN};
use crosec::commands::set_fan_target_rpm::ec_cmd_set_fan_target_rpm;
use crosec::commands::{
    charge_control, get_chip_info::ec_cmd_get_chip_info, hello::ec_cmd_hello,
    version::ec_cmd_version, CrosEcCmd,
};
use crosec::console::console;
use crosec::get_number_of_fans::{get_number_of_fans, Error};
use crosec::read_mem_any::read_mem_any;
use crosec::{
    CROS_EC_PATH, CROS_FP_PATH, EC_FAN_SPEED_ENTRIES, EC_FAN_SPEED_NOT_PRESENT,
    EC_FAN_SPEED_STALLED, EC_MEM_MAP_FAN,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Copy, ValueEnum, Default)]
enum Device {
    #[default]
    Ec,
    Fp,
}

impl Device {
    pub fn get_path(&self) -> &'static str {
        match self {
            Self::Ec => CROS_EC_PATH,
            Self::Fp => CROS_FP_PATH,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Checks for basic communication with EC
    Hello {
        #[arg()]
        device: Option<Device>,
    },
    /// Prints EC version
    Version,
    /// Prints chip info
    ChipInfo,
    /// Prints the board version
    BoardVersion,
    /// Prints supported version mask for a command number
    CmdVersions {
        command: u32,
    },
    /// Set target fan RPM
    SetFanTargetRpm {
        rpm: u32,
        #[arg()]
        index: Option<u8>,
    },
    /// Get supported features
    GetFeatures,
    /// Get number of fans
    GetNumberOfFans,
    /// Get the speed of fans, in RPM
    GetFanRpm,
    /// Prints the last output to the EC debug console
    Console {
        #[arg()]
        device: Option<Device>,
    },
    /// Prints battery info
    Battery,
    ChargeControl {
        #[command(subcommand)]
        command: Option<ChargeControlSubcommands>,
    },
    FpInfo,
    FpStats,
    FpSetSeed {
        seed: String,
    },
    FpMode {
        mode: Vec<String>,
    },
    WaitEvent {
        event_type: String,
        /// Timeout in milliseconds
        timeout: i32,
        device: Option<Device>,
    },
}

#[derive(Subcommand)]
enum ChargeControlSubcommands {
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

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { device } => {
            let file = File::open(device.unwrap_or_default().get_path()).unwrap();
            let fd = file.as_raw_fd();
            let status = ec_cmd_hello(fd)?;
            if status {
                println!("EC says hello!");
            } else {
                println!("EC did not say hello :(");
            }
        }
        Commands::Version => {
            let file = File::open("/dev/cros_ec").unwrap();
            let fd = file.as_raw_fd();
            let max_sizes = get_protocol_info(fd)?;
            let (ro_ver, rw_ver, firmware_copy, build_info, tool_version) =
                ec_cmd_version(fd, &max_sizes)?;
            println!("RO version:    {ro_ver}");
            println!("RW version:    {rw_ver}");
            println!("Firmware copy: {firmware_copy}");
            println!("Build info:    {build_info}");
            println!("Tool version:  {tool_version}");
        }
        Commands::ChipInfo => {
            let file = File::open("/dev/cros_ec").unwrap();
            let fd = file.as_raw_fd();
            let (vendor, name, revision) = ec_cmd_get_chip_info(fd)?;
            println!("Chip info:");
            println!("  vendor:    {vendor}");
            println!("  name:      {name}");
            println!("  revision:  {revision}");
        }
        Commands::BoardVersion => {
            let file = File::open("/dev/cros_ec").unwrap();
            let fd = file.as_raw_fd();
            let board_version = ec_cmd_board_version(fd)?;
            println!("Board version: {board_version}");
        }
        Commands::CmdVersions { command } => match CrosEcCmd::from_u32(command) {
            Some(cmd) => {
                let file = File::open("/dev/cros_ec").unwrap();
                let fd = file.as_raw_fd();
                let versions = ec_cmd_get_cmd_versions(fd, cmd)?;
                println!("Versions: {versions:#b}");
            }
            None => {
                println!("Unknown Command");
            }
        },
        Commands::SetFanTargetRpm { rpm, index } => {
            let file = File::open("/dev/cros_ec").unwrap();
            let fd = file.as_raw_fd();
            ec_cmd_set_fan_target_rpm(fd, rpm, index)?;
            match index {
                Some(index) => {
                    println!("Set RPM to {rpm} for fan {index}");
                }
                None => {
                    println!("Set RPM to {rpm} for all fans");
                }
            }
        }
        Commands::GetFeatures => {
            let file = File::open("/dev/cros_ec").unwrap();
            let fd = file.as_raw_fd();
            let features = ec_cmd_get_features(fd)?;
            println!("EC supported features: {features:#b}");
        }
        Commands::GetNumberOfFans => {
            let file = File::open("/dev/cros_ec").unwrap();
            let fd = file.as_raw_fd();
            let number_of_fans = get_number_of_fans(fd).unwrap();
            println!("Number of fans: {number_of_fans}");
        }
        Commands::GetFanRpm => {
            let file = File::open("/dev/cros_ec").unwrap();
            let fd = file.as_raw_fd();
            let features = ec_cmd_get_features(fd).map_err(|e| Error::GetFeatures(e))?;
            if features & EC_FEATURE_PWM_FAN != 0 {
                read_mem_any::<[u16; EC_FAN_SPEED_ENTRIES]>(fd, EC_MEM_MAP_FAN)
                    .map_err(|e| Error::ReadMem(e))?
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, fan)| match fan {
                        EC_FAN_SPEED_NOT_PRESENT => {}
                        EC_FAN_SPEED_STALLED => {
                            println!("Fan {i} stalled");
                        }
                        fan_speed => {
                            println!("Fan {i} RPM: {fan_speed}");
                        }
                    });
            } else {
                println!("No fans");
            };
        }
        Commands::Console { device } => {
            let file = File::open(device.unwrap_or_default().get_path()).unwrap();
            let fd = file.as_raw_fd();
            let max_sizes = get_protocol_info(fd)?;
            let console = console(fd, &max_sizes)?;
            let console = console.trim();
            println!("{console}");
        }
        Commands::Battery => {
            let file = File::open("/dev/cros_ec").unwrap();
            let fd = file.as_raw_fd();
            let battery_info = battery(fd)?;
            println!("{battery_info:#?}");
        }
        Commands::ChargeControl { command } => {
            let file = File::open("/dev/cros_ec").unwrap();
            let fd = file.as_raw_fd();
            match command {
                None => {
                    if supports_get_and_sustainer(fd)? {
                        let charge_control = get_charge_control(fd)?;
                        println!("{charge_control:#?}");
                    } else {
                        println!("This EC doesn't support getting charge control");
                    }
                }
                Some(command) => match command {
                    ChargeControlSubcommands::Normal {
                        min_percent,
                        max_percent,
                    } => match min_percent {
                        Some(min_percent) => {
                            let max_percent = max_percent.unwrap_or(min_percent);
                            set_charge_control(
                                fd,
                                charge_control::ChargeControl::Normal(Some(Sustainer {
                                    min_percent: min_percent as i8,
                                    max_percent: max_percent as i8,
                                })),
                            )?;
                            println!("Set charge control to normal with sustainer from {min_percent}% to {max_percent}%");
                        }
                        None => {
                            set_charge_control(fd, charge_control::ChargeControl::Normal(None))?;
                            println!("Set charge control to normal");
                        }
                    },
                    ChargeControlSubcommands::Idle => {
                        println!("Set charge control to idle");
                        set_charge_control(fd, charge_control::ChargeControl::Idle)?;
                    }
                    ChargeControlSubcommands::Discharge => {
                        println!("Set charge control to discharge");
                        set_charge_control(fd, charge_control::ChargeControl::Discharge)?;
                    }
                },
            }
        }
        Commands::FpInfo => {
            let file = File::open(CROS_FP_PATH).unwrap();
            let fd = file.as_raw_fd();
            let info = fp_info(fd)?;
            println!("{info:#?}");
        }
        Commands::FpStats => {
            let file = File::open(CROS_FP_PATH).unwrap();
            let fd = file.as_raw_fd();
            let stats = fp_stats(fd)?;
            println!("{stats:#?}");
        }
        Commands::FpSetSeed { seed } => {
            match <Vec<u8> as TryInto<[u8; FP_CONTEXT_TPM_BYTES]>>::try_into(
                seed.as_bytes().to_owned(),
            ) {
                Ok(seed) => {
                    let file = File::open(CROS_FP_PATH).unwrap();
                    let fd = file.as_raw_fd();
                    fp_set_seed(fd, seed)?;
                    println!("Set fp seed");
                }
                Err(seed) => {
                    let seed_len = seed.len();
                    println!("The seed must be {FP_CONTEXT_TPM_BYTES} bytes long. The seed you inputted is {seed_len} bytes long.");
                }
            }
        }
        Commands::FpMode { mode } => {
            let mode = if mode.len() > 0 {
                let mut mode_number: u32 = 0;
                for mode in mode {
                    mode_number |= FpMode::from_str(&mode)? as u32;
                }
                mode_number
            } else {
                FpMode::DontChange as u32
            };
            let file = File::open(CROS_FP_PATH).unwrap();
            let fd = file.as_raw_fd();
            let mode = fp_mode(fd, mode)?;
            let display = FpMode::display(mode);
            println!("FP mode: {display}");
        }
        Commands::WaitEvent {
            event_type,
            device,
            timeout,
        } => {
            let mut file = File::open(device.unwrap_or_default().get_path())?;
            let result =
                wait_event(&mut file, EcMkbpEventType::from_str(&event_type)?, timeout).unwrap();
            println!("{result:#?}");
        }
    }

    Ok(())
}
