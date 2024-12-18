#![warn(unused_crate_dependencies)]

use std::fs::File;

use charge_control_subcommand::{charge_control_subcommand, ChargeControlSubcommand};
use charge_current_limit_subcommand::charge_current_limit_subcommand;
use check_seed::check_seed;
use check_user_id::check_user_id;
use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use crosec::commands::fp_info::fp_info;
use crosec::commands::fp_mode::{fp_mode, FpMode};
use crosec::commands::fp_set_context::UserId;
use crosec::commands::fp_set_seed::{fp_set_seed, FP_CONTEXT_TPM_BYTES};
use crosec::commands::fp_stats::fp_stats;
use crosec::commands::get_protocol_info::get_protocol_info;
use crosec::wait_event::{event::EcMkbpEventType, wait_event_sync};
use fp_download_subcommand::{fp_download_subcommand, FpDownloadSubcommand};
use fp_set_context_command::fp_context_command;
use fp_upload_template_command::fp_upload_template_command;
use get_uptime_info_command::get_uptime_info_commnad;
use num_traits::cast::FromPrimitive;
use strum::IntoEnumIterator;

use crate::fp_get_encryption_status_command::fp_get_encryption_status_command;
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
mod charge_current_limit_subcommand;
mod check_seed;
mod check_user_id;
mod fp_download_subcommand;
mod fp_get_encryption_status_command;
mod fp_set_context_command;
mod fp_upload_template_command;
mod get_uptime_info_command;

#[derive(Parser)]
#[command(version, about)]
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
        #[arg(value_parser = check_seed)]
        seed: [u8; FP_CONTEXT_TPM_BYTES],
    },
    FpMode {
        mode: Vec<FpMode>,
    },
    WaitEvent {
        event_types: Vec<EcMkbpEventType>,
        /// Timeout in milliseconds
        #[arg(short, long)]
        timeout: Option<i32>,
        #[arg(short, long)]
        device: Option<Device>,
    },
    FpDownload {
        #[command(subcommand)]
        command: FpDownloadSubcommand,
    },
    /// Uploads template from stdin
    FpUploadTemplate,
    FpGetEncryptionStatus,
    GetUptimeInfo {
        device: Option<Device>,
    },
    ChargeCurrentLimit {
        /// Limit in mA
        #[arg()]
        limit: u32,
    },
    FpSetContext {
        /// A 32 byte hex string
        #[arg(value_parser = check_user_id)]
        user_id: UserId,
    },
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
            let features = ec_cmd_get_features(&mut file).map_err(Error::GetFeatures)?;
            if features & EC_FEATURE_PWM_FAN != 0 {
                read_mem_any::<[u16; EC_FAN_SPEED_ENTRIES]>(&mut file, EC_MEM_MAP_FAN)
                    .map_err(Error::ReadMem)?
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
            let mut file = File::open(CROS_FP_PATH)?;
            fp_set_seed(&mut file, seed)?;
            println!("Set fp seed");
        }
        Commands::FpMode { mode } => {
            let mode = if !mode.is_empty() {
                let mut mode_number: u32 = 0;
                for mode in mode {
                    mode_number |= mode as u32;
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
            mut event_types,
            device,
            timeout,
        } => {
            if event_types.is_empty() {
                event_types = EcMkbpEventType::iter().collect();
            }
            let mut file = File::open(device.unwrap_or_default().get_path())?;
            println!("Waiting for event...");
            let result = wait_event_sync(&mut file, event_types, timeout).unwrap();
            println!("{result:#?}");
        }
        Commands::FpDownload { command } => fp_download_subcommand(command)?,
        Commands::FpUploadTemplate => fp_upload_template_command()?,
        Commands::FpGetEncryptionStatus => fp_get_encryption_status_command()?,
        Commands::GetUptimeInfo { device } => get_uptime_info_commnad(device)?,
        Commands::ChargeCurrentLimit { limit } => charge_current_limit_subcommand(limit)?,
        Commands::FpSetContext { user_id } => fp_context_command(user_id)?,
    }

    Ok(())
}
