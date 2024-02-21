use crosec_rs::commands::hello::ec_cmd_hello;
use crosec_rs::commands::version::ec_cmd_version;

fn main() {
    println!("hello");
    ec_cmd_hello();
    ec_cmd_version();
}
