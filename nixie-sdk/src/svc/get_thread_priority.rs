use super::{Handle, Thread};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub fn get_thread_priority(thread: Handle<Thread>) -> Result<u32, ResultCode> {
  let mut error_code: u32;
  let mut priority: u32;

  unsafe {
    asm!(
      "svc #0x0c",

      in("x0") thread.as_bits(),
      lateout("x0") error_code,
      lateout("x1") priority,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(priority);
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}

#[cfg(target_pointer_width = "32")]
pub fn get_thread_priority(thread: Handle<Thread>) -> Result<u32, ResultCode> {
  let mut error_code: u32;
  let mut priority: u32;

  unsafe {
    asm!(
      "svc #0x0c",

      in("w0") thread.as_bits(),
      lateout("w0") error_code,
      lateout("w1") priority,
      lateout("w2") _,
      lateout("w3") _,
    );
  }

  if error_code == 0 {
    return Ok(priority);
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
