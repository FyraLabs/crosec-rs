use color_eyre::eyre::Result;
use crosec::commands::{
    get_chip_info::ec_cmd_get_chip_info, hello::ec_cmd_hello, version::ec_cmd_version,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Hello command");
    let status = ec_cmd_hello()?;
    if status {
        println!("EC says hello!");
    } else {
        println!("EC did not say hello :(");
    }

    println!("Version command");
    let (ro_ver, rw_ver, firmware_copy, build_info, tool_version) = ec_cmd_version()?;
    println!("RO version:    {ro_ver}");
    println!("RW version:    {rw_ver}");
    println!("Firmware copy: {firmware_copy}");
    println!("Build info:    {build_info}");
    println!("Tool version:  {tool_version}");

    println!("Chip info command");
    let (vendor, name, revision) = ec_cmd_get_chip_info()?;
    println!("Chip info:");
    println!("  vendor:    {vendor}");
    println!("  name:      {name}");
    println!("  revision:  {revision}");

    Ok(())
}