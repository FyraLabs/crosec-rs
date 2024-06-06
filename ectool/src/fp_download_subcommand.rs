use std::{
    fs::File,
    io::{stdout, Write},
};

use clap::{Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use crosec::{
    commands::{
        fp_download::{fp_download, fp_download_template, DownloadType},
        fp_info::fp_info,
        get_protocol_info::get_protocol_info,
    },
    CROS_FP_PATH,
};
use image::{
    codecs::pnm::{PnmEncoder, PnmSubtype, SampleEncoding},
    ImageEncoder,
};

#[derive(ValueEnum, Clone, Copy, Default)]
pub enum FrameType {
    Raw,
    #[default]
    Pgm,
}

#[derive(Subcommand)]
pub enum FpDownloadSubcommand {
    Frame { frame_type: Option<FrameType> },
    Template { index: usize },
}

pub fn fp_download_subcommand(command: FpDownloadSubcommand) -> Result<()> {
    let mut file = File::open(CROS_FP_PATH)?;
    let fp_info = fp_info(&mut file)?;
    let protocol_info = get_protocol_info(&mut file)?;
    match command {
        FpDownloadSubcommand::Frame { frame_type } => match frame_type.unwrap_or_default() {
            FrameType::Raw => {
                let frame =
                    fp_download(&mut file, &fp_info, &protocol_info, &DownloadType::RawImage);
                stdout().write_all(&frame)?;
            }
            FrameType::Pgm => {
                let frame = fp_download(
                    &mut file,
                    &fp_info,
                    &protocol_info,
                    &DownloadType::SimpleImage,
                );
                PnmEncoder::new(stdout())
                    .with_subtype(PnmSubtype::Graymap(SampleEncoding::Binary))
                    .write_image(
                        &frame,
                        fp_info.width as u32,
                        fp_info.height as u32,
                        image::ExtendedColorType::L8,
                    )?;
            }
        },
        FpDownloadSubcommand::Template { index } => {
            let template = fp_download_template(&mut file, &fp_info, &protocol_info, index);
            stdout().write_all(template.buffer())?;
        }
    }
    Ok(())
}
