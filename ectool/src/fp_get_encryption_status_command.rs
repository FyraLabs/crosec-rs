use std::fs::File;
use crosec::commands::fp_get_encryption_status::{EcResponseFpGetEncryptionStatus, fp_get_encryption_status};
use crosec::CROS_FP_PATH;

pub fn fp_get_encryption_status_command() -> color_eyre::Result<()> {
    let mut file = File::open(CROS_FP_PATH)?;
    let EcResponseFpGetEncryptionStatus { status, valid_flags } = fp_get_encryption_status(&mut file)?;
    println!("FPMCU encryption status: {status:#b}");
    println!("Valid flags:             {valid_flags:#b}");
    Ok(())
}
