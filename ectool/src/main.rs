use std::env;
use seahorse::{App, Command};
use crosec::commands::{
    get_chip_info::ec_cmd_get_chip_info, hello::ec_cmd_hello, version::ec_cmd_version,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .command(Command::new("hello").action(|_c| {
            let status = ec_cmd_hello().unwrap();
            if status {
                println!("EC says hello!");
            } else {
                println!("EC did not say hello :(");
            }
        }))
        .command(Command::new("version").action(|_c| {
            let (ro_ver, rw_ver, firmware_copy, build_info, tool_version) = ec_cmd_version().unwrap();
            println!("RO version:    {ro_ver}");
            println!("RW version:    {rw_ver}");
            println!("Firmware copy: {firmware_copy}");
            println!("Build info:    {build_info}");
            println!("Tool version:  {tool_version}");
        }))
        .command(Command::new("chipinfo").action(|_c| {
            let (vendor, name, revision) = ec_cmd_get_chip_info().unwrap();
            println!("Chip info:");
            println!("  vendor:    {vendor}");
            println!("  name:      {name}");
            println!("  revision:  {revision}");
        }));

    app.run(args);
}