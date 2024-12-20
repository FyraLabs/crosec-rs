use crosec::commands::fp_set_context::UserId;

pub fn check_user_id(user_id_str: &str) -> Result<UserId, String> {
    let mut user_id = UserId::default();
    hex::decode_to_slice(user_id_str.as_bytes(), &mut user_id).map_err(|e| e.to_string())?;
    Ok(user_id)
}
