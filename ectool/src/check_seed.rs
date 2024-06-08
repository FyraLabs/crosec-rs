use crosec::commands::fp_set_seed::FP_CONTEXT_TPM_BYTES;

pub fn check_seed(seed: &str) -> Result<[u8; FP_CONTEXT_TPM_BYTES], String> {
    match <Vec<u8> as TryInto<[u8; FP_CONTEXT_TPM_BYTES]>>::try_into(seed.as_bytes().to_owned()) {
        Ok(seed) => Ok(seed),
        Err(seed) => {
            let seed_len = seed.len();
            Err(format!("The seed must be {FP_CONTEXT_TPM_BYTES} bytes long. The seed you inputted is {seed_len} bytes long."))
        }
    }
}
