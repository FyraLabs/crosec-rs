use std::fs::File;

use crosec::commands::get_uptime_info::ec_cmd_get_uptime_info;

use crate::Device;

pub fn get_uptime_info_commnad(device: Option<Device>) -> color_eyre::Result<()> {
    let mut file = File::open(device.unwrap_or_default().get_path())?;
    let uptime_info = ec_cmd_get_uptime_info(&mut file)?;
    println!("Uptime info: {uptime_info:#?}");
    Ok(())
}
