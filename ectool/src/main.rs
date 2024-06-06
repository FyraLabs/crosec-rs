use std::fs::File;
use std::str::FromStr;

use charge_control_subcommand::{charge_control_subcommand, ChargeControlSubcommand};
use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use crosec::commands::fp_info::fp_info;
use crosec::commands::fp_mode::{fp_mode, FpMode};
use crosec::commands::fp_set_seed::{fp_set_seed, FP_CONTEXT_TPM_BYTES};
use crosec::commands::fp_stats::fp_stats;
use crosec::commands::get_protocol_info::get_protocol_info;
use crosec::wait_event::{event::EcMkbpEventType, wait_event};
use fp_download_subcommand::{fp_download_subcommand, FpDownloadSubcommand};
use fp_upload_template_command::fp_upload_template_command;
use num_traits::cast::FromPrimitive;

use crosec::battery::battery;
use crosec::commands::board_version::ec_cmd_board_version;
use crosec::commands::get_cmd_versions::ec_cmd_get_cmd_versions;
use crosec::commands::get_features::{ec_cmd_get_features, EC_FEATURE_PWM_FAN};
use crosec::commands::set_fan_target_rpm::ec_cmd_set_fan_target_rpm;
use crosec::commands::{
    get_chip_info::ec_cmd_get_chip_info, hello::ec_cmd_hello, version::ec_cmd_version, CrosEcCmd,
};
use crosec::console::console;
use crosec::get_number_of_fans::{get_number_of_fans, Error};
use crosec::read_mem_any::read_mem_any;
use crosec::{
    CROS_EC_PATH, CROS_FP_PATH, EC_FAN_SPEED_ENTRIES, EC_FAN_SPEED_NOT_PRESENT,
    EC_FAN_SPEED_STALLED, EC_MEM_MAP_FAN,
};

mod charge_control_subcommand;
mod fp_download_subcommand;
mod fp_upload_template_command;

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
        command: Option<ChargeControlSubcommand>,
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
    FpDownload {
        #[command(subcommand)]
        command: FpDownloadSubcommand,
    },
    /// Uploads template from stdin
    FpUploadTemplate,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { device } => {
            let mut file = File::open(device.unwrap_or_default().get_path())?;
            let status = ec_cmd_hello(&mut file)?;
            if status {
                println!("EC says hello!");
            } else {
                println!("EC did not say hello :(");
            }
        }
        Commands::Version => {
            let mut file = File::open(CROS_EC_PATH)?;
            let max_sizes = get_protocol_info(&mut file)?;
            let (ro_ver, rw_ver, firmware_copy, build_info, tool_version) =
                ec_cmd_version(&mut file, &max_sizes)?;
            println!("RO version:    {ro_ver}");
            println!("RW version:    {rw_ver}");
            println!("Firmware copy: {firmware_copy}");
            println!("Build info:    {build_info}");
            println!("Tool version:  {tool_version}");
        }
        Commands::ChipInfo => {
            let mut file = File::open(CROS_EC_PATH)?;
            let (vendor, name, revision) = ec_cmd_get_chip_info(&mut file)?;
            println!("Chip info:");
            println!("  vendor:    {vendor}");
            println!("  name:      {name}");
            println!("  revision:  {revision}");
        }
        Commands::BoardVersion => {
            let mut file = File::open(CROS_EC_PATH)?;
            let board_version = ec_cmd_board_version(&mut file)?;
            println!("Board version: {board_version}");
        }
        Commands::CmdVersions { command } => match CrosEcCmd::from_u32(command) {
            Some(cmd) => {
                let mut file = File::open(CROS_EC_PATH)?;
                let versions = ec_cmd_get_cmd_versions(&mut file, cmd)?;
                println!("Versions: {versions:#b}");
            }
            None => {
                println!("Unknown Command");
            }
        },
        Commands::SetFanTargetRpm { rpm, index } => {
            let mut file = File::open(CROS_EC_PATH)?;
            ec_cmd_set_fan_target_rpm(&mut file, rpm, index)?;
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
            let mut file = File::open(CROS_EC_PATH)?;
            let features = ec_cmd_get_features(&mut file)?;
            println!("EC supported features: {features:#b}");
        }
        Commands::GetNumberOfFans => {
            let mut file = File::open(CROS_EC_PATH)?;
            let number_of_fans = get_number_of_fans(&mut file).unwrap();
            println!("Number of fans: {number_of_fans}");
        }
        Commands::GetFanRpm => {
            let mut file = File::open(CROS_EC_PATH)?;
            let features = ec_cmd_get_features(&mut file).map_err(|e| Error::GetFeatures(e))?;
            if features & EC_FEATURE_PWM_FAN != 0 {
                read_mem_any::<[u16; EC_FAN_SPEED_ENTRIES]>(&mut file, EC_MEM_MAP_FAN)
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
            let mut file = File::open(device.unwrap_or_default().get_path())?;
            let max_sizes = get_protocol_info(&mut file)?;
            let console = console(&mut file, &max_sizes)?;
            let console = console.trim();
            println!("{console}");
        }
        Commands::Battery => {
            let mut file = File::open(CROS_EC_PATH)?;
            let battery_info = battery(&mut file)?;
            println!("{battery_info:#?}");
        }
        Commands::ChargeControl { command } => charge_control_subcommand(command)?,
        Commands::FpInfo => {
            let mut file = File::open(CROS_FP_PATH)?;
            let info = fp_info(&mut file)?;
            println!("{info:#?}");
        }
        Commands::FpStats => {
            let mut file = File::open(CROS_FP_PATH)?;
            let stats = fp_stats(&mut file)?;
            println!("{stats:#?}");
        }
        Commands::FpSetSeed { seed } => {
            match <Vec<u8> as TryInto<[u8; FP_CONTEXT_TPM_BYTES]>>::try_into(
                seed.as_bytes().to_owned(),
            ) {
                Ok(seed) => {
                    let mut file = File::open(CROS_FP_PATH)?;
                    fp_set_seed(&mut file, seed)?;
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
            let mut file = File::open(CROS_FP_PATH)?;
            let mode = fp_mode(&mut file, mode)?;
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
        Commands::FpDownload { command } => fp_download_subcommand(command)?,
        Commands::FpUploadTemplate => fp_upload_template_command()?,
    }

    Ok(())
}
