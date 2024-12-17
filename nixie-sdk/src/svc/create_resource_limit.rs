use super::{Handle, ResourceLimit};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub fn create_resource_limit() -> Result<Handle<ResourceLimit>, ResultCode> {
  let mut error_code: u32;
  let mut resource_limit_handle: u32;

  unsafe {
    asm!(
      "svc #0x7D",

      lateout("w0") error_code,
      lateout("w1") resource_limit_handle,
      lateout("w2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(unsafe { Handle::<ResourceLimit>::from_bits(resource_limit_handle) });
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
