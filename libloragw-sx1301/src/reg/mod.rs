use super::{mutex, wrapper};

/// LoRa concentrator register write.
pub fn reg_w(reg_id: u32, reg_value: i32) -> Result<(), String> {
    let _guard = mutex::CONCENTATOR.lock().unwrap();
    let ret = unsafe { wrapper::lgw_reg_w(reg_id as u16, reg_value) };
    if ret != 0 {
        return Err("lgw_reg_w failed".to_string());
    }

    return Ok(());
}
