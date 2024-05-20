use std::fs::File;
use std::os::fd::AsRawFd;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use num_traits::cast::FromPrimitive;

use crosec::commands::{CrosEcCmd, get_chip_info::ec_cmd_get_chip_info, hello::ec_cmd_hello, version::ec_cmd_version};
use crosec::commands::board_version::ec_cmd_board_version;
use crosec::commands::get_cmd_versions::ec_cmd_get_cmd_versions;
use crosec::commands::get_features::{ec_cmd_get_features, EC_FEATURE_PWM_FAN};
use crosec::commands::set_fan_target_rpm::ec_cmd_set_fan_target_rpm;
use crosec::{EC_FAN_SPEED_ENTRIES, EC_FAN_SPEED_NOT_PRESENT, EC_FAN_SPEED_STALLED, EC_MEM_MAP_FAN};
use crosec::get_number_of_fans::{Error, get_number_of_fans};
use crosec::read_mem_any::read_mem_any;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Checks for basic communication with EC
    Hello,
    /// Prints EC version
    Version,
    /// Prints chip info
    ChipInfo,
    /// Prints the board version
    BoardVersion,
    /// Prints supported version mask for a command number
    CmdVersions {
        command: u32
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
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let file = File::open("/dev/cros_ec").unwrap();
    let fd = file.as_raw_fd();

    match cli.command {
        Commands::Hello => {
            let status = ec_cmd_hello(fd)?;
            if status {
                println!("EC says hello!");
            } else {
                println!("EC did not say hello :(");
            }
        }
        Commands::Version => {
            let (ro_ver, rw_ver, firmware_copy, build_info, tool_version) = ec_cmd_version(fd)?;
            println!("RO version:    {ro_ver}");
            println!("RW version:    {rw_ver}");
            println!("Firmware copy: {firmware_copy}");
            println!("Build info:    {build_info}");
            println!("Tool version:  {tool_version}");
        }
        Commands::ChipInfo => {
            let (vendor, name, revision) = ec_cmd_get_chip_info(fd)?;
            println!("Chip info:");
            println!("  vendor:    {vendor}");
            println!("  name:      {name}");
            println!("  revision:  {revision}");
        }
        Commands::BoardVersion => {
            let board_version = ec_cmd_board_version(fd)?;
            println!("Board version: {board_version}");
        }
        Commands::CmdVersions { command } => {
            match CrosEcCmd::from_u32(command) {
                Some(cmd) => {
                    let versions = ec_cmd_get_cmd_versions(fd, cmd)?;
                    println!("Versions: {versions:#b}");
                }
                None => {
                    println!("Unknown Command");
                }
            }
        }
        Commands::SetFanTargetRpm { rpm, index } => {
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
            let features = ec_cmd_get_features(fd)?;
            println!("EC supported features: {features:#b}");
        }
        Commands::GetNumberOfFans => {
            let number_of_fans = get_number_of_fans(fd).unwrap();
            println!("Number of fans: {number_of_fans}");
        }
        Commands::GetFanRpm => {
            let features = ec_cmd_get_features(fd).map_err(|e| Error::GetFeatures(e))?;
            if features & EC_FEATURE_PWM_FAN != 0 {
                read_mem_any::<[u16; EC_FAN_SPEED_ENTRIES]>(fd, EC_MEM_MAP_FAN).map_err(|e| Error::ReadMem(e))?
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
    }

    Ok(())
}