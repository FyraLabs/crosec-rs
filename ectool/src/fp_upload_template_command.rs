use std::{
    fs::File,
    io::{stdin, Read},
};

use color_eyre::eyre::Result;
use crosec::{
    commands::{
        fp_download::FpTemplate, fp_info::fp_info, fp_upload_template::fp_upload_template,
        get_protocol_info::get_protocol_info,
    },
    CROS_FP_PATH,
};

pub fn fp_upload_template_command() -> Result<()> {
    let mut buf = Default::default();
    println!("Reading from stdin. If this command is taking a long time, it's probably because there is no EOF inputted from stdin.");
    stdin().read_to_end(&mut buf)?;
    let template = unsafe { FpTemplate::from_vec_unchecked(buf) };
    let mut file = File::open(CROS_FP_PATH)?;
    let protocol_info = get_protocol_info(&mut file)?;
    let fp_info = fp_info(&mut file)?;
    fp_upload_template(&mut file, &protocol_info, &fp_info, &template)?;
    println!("Uploaded template");
    Ok(())
}
