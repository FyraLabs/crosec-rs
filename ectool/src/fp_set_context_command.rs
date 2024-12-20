use crosec::commands::fp_set_context::{fp_set_context, UserId};
use crosec::CROS_FP_PATH;
use std::fs::File;

pub fn fp_context_command(user_id: UserId) -> color_eyre::Result<()> {
    let mut file = File::open(CROS_FP_PATH)?;
    fp_set_context(&mut file, user_id)?;
    let user_id_str = hex::encode(user_id);
    println!("Set FP context to user id: 0x{user_id_str}");
    Ok(())
}
