use crate::result::result_code::ResultCode;
use core::arch::asm;

use super::{Debug, Handle};

#[cfg(target_pointer_width = "64")]
pub fn get_thread_list(handle: Handle<Debug>, thread_ids: &mut [u64]) -> Result<u64, ResultCode> {
  let mut error_code: u32;
  let mut thread_count: u64;

  unsafe {
    asm!(
      "svc #0x66",

      in("x1") thread_ids.as_ptr(),
      in("x2") thread_ids.len(),
      in("x3") handle.as_bits(),
      lateout("x0") error_code,
      lateout("x1") thread_count,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(thread_count);
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
