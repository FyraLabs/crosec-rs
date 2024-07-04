use std::{os::fd::AsRawFd, thread::sleep, time::Duration};

use bytemuck::{bytes_of, Pod, Zeroable};

use crate::ec_command::ec_command_with_dynamic_output_size;

use super::{fp_info::EcResponseFpInfo, get_protocol_info::EcResponseGetProtocolInfo, CrosEcCmd};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct EcParamsFpFrame {
    /// The offset contains the template index or FP_FRAME_INDEX_RAW_IMAGE
    /// in the high nibble, and the real offset within the frame in
    /// FP_FRAME_OFFSET_MASK.
    offset: u32,
    size: u32,
}

const FP_FRAME_INDEX_RAW_IMAGE: u32 = 0;

/// This can be changed. `3` is what the ChromiumOS ectool uses.
const MAX_ATTEMPTS: usize = 3;

pub enum DownloadType {
    /// (aka `FP_FRAME_INDEX_SIMPLE_IMAGE`) for the a single grayscale image
    SimpleImage,
    /// (aka `FP_FRAME_INDEX_RAW_IMAGE`) for the full vendor raw finger image.
    RawImage,
    Template(usize),
}

/// Downloads a frame buffer from the FPMCU.
/// The downloaded data might be either the finger image or a finger template.
pub fn fp_download<File: AsRawFd>(
    file: &mut File,
    fp_info: &EcResponseFpInfo,
    protocol_info: &EcResponseGetProtocolInfo,
    download_type: &DownloadType,
) -> Vec<u8> {
    let (size, index) = match download_type {
        DownloadType::SimpleImage => (fp_info.get_simple_image_size(), FP_FRAME_INDEX_RAW_IMAGE),
        DownloadType::RawImage => (fp_info.frame_size as usize, FP_FRAME_INDEX_RAW_IMAGE),
        DownloadType::Template(template_index) => {
            (fp_info.template_size as usize, *template_index as u32 + 1)
        }
    };
    // The template may be (and probably is) bigger than the max output size, so we need to download it in chunks
    let number_of_chunks = size.div_ceil(protocol_info.max_ec_output_size());
    let mut chunks = Vec::<Vec<u8>>::with_capacity(number_of_chunks);
    for chunk_index in 0..number_of_chunks {
        let bytes_read = chunk_index * protocol_info.max_ec_output_size();
        let remaining_bytes = size - bytes_read;
        let current_chunk_size = remaining_bytes.min(protocol_info.max_ec_output_size());
        let mut attempt = 0;
        loop {
            let result = ec_command_with_dynamic_output_size(
                CrosEcCmd::FpFrame,
                0,
                bytes_of(&EcParamsFpFrame {
                    offset: (index << 28) + (bytes_read as u32),
                    size: current_chunk_size as u32,
                }),
                current_chunk_size,
                file.as_raw_fd(),
            );
            if let Ok(chunk) = result {
                chunks.push(chunk);
                break;
            } else {
                attempt += 1;
                if attempt == MAX_ATTEMPTS {
                    panic!("Could not successfully get the fp frame in {MAX_ATTEMPTS} attempts");
                }
                // Using micros and not millis to be more like original `usleep(100000)` from ChromiumOS's ectool
                sleep(Duration::from_micros(100_000));
            }
        }
    }
    chunks.concat()
}

/// A safe wrapper around the actual template so you don't try to upload arbitrary data
pub struct FpTemplate {
    vec: Vec<u8>,
}

impl From<FpTemplate> for Vec<u8> {
    fn from(value: FpTemplate) -> Self {
        value.vec
    }
}

impl FpTemplate {
    pub fn buffer(&self) -> &Vec<u8> {
        &self.vec
    }

    /// Make sure your buffer is actually a compatible fp template
    /// # Safety
    /// Make sure you're uploading a template from the same FPMCU with the same version and the same seed set.
    pub unsafe fn from_vec_unchecked(vec: Vec<u8>) -> Self {
        FpTemplate { vec }
    }
}

pub fn fp_download_template<File: AsRawFd>(
    file: &mut File,
    fp_info: &EcResponseFpInfo,
    protocol_info: &EcResponseGetProtocolInfo,
    index: usize,
) -> FpTemplate {
    FpTemplate {
        vec: fp_download(file, fp_info, protocol_info, &DownloadType::Template(index)),
    }
}
