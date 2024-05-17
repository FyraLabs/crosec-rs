use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use crosec::commands::{CrosEcCmd, get_chip_info::ec_cmd_get_chip_info, hello::ec_cmd_hello, version::ec_cmd_version};
use crosec::commands::board_version::ec_cmd_board_version;
use crosec::commands::get_cmd_versions::ec_cmd_get_cmd_versions;
use num_traits::cast::FromPrimitive;
use crosec::commands::set_fan_target_rpm::ec_cmd_set_fan_target_rpm;

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
        index: Option<u8>
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => {
            let status = ec_cmd_hello()?;
            if status {
                println!("EC says hello!");
            } else {
                println!("EC did not say hello :(");
            }
        }
        Commands::Version => {
            let (ro_ver, rw_ver, firmware_copy, build_info, tool_version) = ec_cmd_version()?;
            println!("RO version:    {ro_ver}");
            println!("RW version:    {rw_ver}");
            println!("Firmware copy: {firmware_copy}");
            println!("Build info:    {build_info}");
            println!("Tool version:  {tool_version}");
        }
        Commands::ChipInfo => {
            let (vendor, name, revision) = ec_cmd_get_chip_info()?;
            println!("Chip info:");
            println!("  vendor:    {vendor}");
            println!("  name:      {name}");
            println!("  revision:  {revision}");
        },
        Commands::BoardVersion => {
            let board_version = ec_cmd_board_version()?;
            println!("Board version: {board_version}");
        },
        Commands::CmdVersions {command} => {
            match CrosEcCmd::from_u32(command) {
                Some(cmd) => {
                    let versions = ec_cmd_get_cmd_versions(cmd)?;
                    println!("Versions: {versions:#b}");
                },
                None => {
                    println!("Unknown Command");
                }
            }
        },
        Commands::SetFanTargetRpm {rpm, index} => {
            ec_cmd_set_fan_target_rpm(rpm, index)?;
            match index {
                Some(index) => {
                    println!("Set RPM to {rpm} for fan {index}");
                },
                None => {
                    println!("Set RPM to {rpm} for all fans");
                }
            }
        }
    }

    Ok(())
}