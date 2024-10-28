use std::{mem::offset_of, os::fd::AsRawFd};

use bytemuck::{bytes_of, Pod, Zeroable};

use crate::{ec_command::ec_command_with_dynamic_output_size, EcCmdResult};

use super::{
    fp_download::FpTemplate, fp_info::EcResponseFpInfo,
    get_protocol_info::EcResponseGetProtocolInfo, CrosEcCmd,
};

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct EcParamsFpTemplateWithoutData {
    offset: u32,
    size: u32,
    data: [u8; 0],
}

/// Flag in the 'size' field indicating that the full template has been sent
const FP_TEMPLATE_COMMIT: u32 = 0x80000000;

pub fn fp_upload_template<File: AsRawFd>(
    file: &mut File,
    protocol_info: &EcResponseGetProtocolInfo,
    fp_info: &EcResponseFpInfo,
    template: &FpTemplate,
) -> EcCmdResult<()> {
    assert_eq!(
        template.buffer().len(),
        fp_info.template_size as usize,
        "The given template must match the fp sensor's template size"
    );
    // TODO(b/78544921): removing 32 bits is a workaround for the MCU bug
    // Idk what this bug is, but the ChromiumOS ectool removes 4 bytes, so we should too
    let max_chunk_size =
        protocol_info.max_ec_output_size() - offset_of!(EcParamsFpTemplateWithoutData, data) - 4;
    let number_of_chunks = template
        .buffer()
        .len()
        .div_ceil(protocol_info.max_ec_input_size());
    for chunk_index in 0..number_of_chunks {
        ec_command_with_dynamic_output_size(
            CrosEcCmd::FpTemplate,
            0,
            &{
                let bytes_uploaded = chunk_index * max_chunk_size;
                let size = (template.buffer().len() - bytes_uploaded).min(max_chunk_size);
                let mut vec = bytes_of(&EcParamsFpTemplateWithoutData {
                    offset: bytes_uploaded as u32,
                    size: {
                        let mut size = size as u32;
                        if chunk_index == number_of_chunks - 1 {
                            size |= FP_TEMPLATE_COMMIT;
                        }
                        size
                    },
                    data: [],
                })
                .to_vec();
                vec.extend_from_slice(&template.buffer()[bytes_uploaded..bytes_uploaded + size]);
                vec
            },
            0,
            file.as_raw_fd(),
        )?;
    }
    Ok(())
}
