use core::arch::asm;

use crate::result::result_code::ResultCode;

use super::{Handle, ResourceLimit};


#[cfg(target_pointer_width = "64")]
fn get_resource_limit_current_value_raw(handle_bits: u32, limitable_resource: u32) -> Result<u64, ResultCode> {
  let mut error_code: usize;
  let mut result: u64;

  unsafe {
    asm!(
      "svc #0x31",
      
      in("w1") handle_bits,
      in("w2") limitable_resource,
      lateout("x0") error_code,
      lateout("x1") result,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(result);
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}

pub fn get_max_physical_memory_resource_limit_current_value(handle: Handle<ResourceLimit>) -> Result<u64, ResultCode> { get_resource_limit_current_value_raw(handle.as_bits(), 0) }
pub fn get_max_thread_count_resource_limit_current_value(handle: Handle<ResourceLimit>) -> Result<u64, ResultCode> { get_resource_limit_current_value_raw(handle.as_bits(), 1) }
pub fn get_max_event_count_resource_limit_current_value(handle: Handle<ResourceLimit>) -> Result<u64, ResultCode> { get_resource_limit_current_value_raw(handle.as_bits(), 2) }
pub fn get_max_transfer_memory_count_resource_limit_current_value(handle: Handle<ResourceLimit>) -> Result<u64, ResultCode> { get_resource_limit_current_value_raw(handle.as_bits(), 3) }
pub fn get_max_session_count_max_resource_limit_current_value(handle: Handle<ResourceLimit>) -> Result<u64, ResultCode> { get_resource_limit_current_value_raw(handle.as_bits(), 4) }