use core::arch::asm;
use crate::result::result_code::ResultCode;
use super::{Handle, ResourceLimit};

#[cfg(target_pointer_width = "64")]
fn set_resource_limit_value_raw(handle: Handle<ResourceLimit>, limitable_resource: u32, limit_value: u64) -> Result<(), ResultCode> {
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x7E",
      
      in("w0") handle.as_bits(),
      in("w1") limitable_resource,
      in("x2") limit_value,
      lateout("w0") error_code,
      lateout("w1") _,
      lateout("w2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(());
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}

pub fn set_max_physical_memory_limit_value(handle: Handle<ResourceLimit>, limit_value: u64) -> Result<(), ResultCode> {
  set_resource_limit_value_raw(handle, 0, limit_value)
}

pub fn set_max_thread_count_limit_value(handle: Handle<ResourceLimit>, limit_value: u64) -> Result<(), ResultCode> {
  set_resource_limit_value_raw(handle, 1, limit_value)
}

pub fn set_max_event_count_limit_value(handle: Handle<ResourceLimit>, limit_value: u64) -> Result<(), ResultCode> {
  set_resource_limit_value_raw(handle, 2, limit_value)
}

pub fn set_max_transfer_memory_count_limit_value(handle: Handle<ResourceLimit>, limit_value: u64) -> Result<(), ResultCode> {
  set_resource_limit_value_raw(handle, 3, limit_value)
}

pub fn set_max_session_count_limit_value(handle: Handle<ResourceLimit>, limit_value: u64) -> Result<(), ResultCode> {
  set_resource_limit_value_raw(handle, 4, limit_value)
}