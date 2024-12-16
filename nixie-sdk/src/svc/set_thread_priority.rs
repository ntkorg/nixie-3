use crate::result::result_code::ResultCode;
use core::arch::asm;
use super::{Handle, Thread};

#[cfg(target_pointer_width = "64")]
pub fn set_thread_priority(thread: Handle<Thread>, priority: u32) -> Result<(), ResultCode> {
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x0d",
      
      in("x0") thread.as_bits(),
      in("x1") priority,
      lateout("x0") error_code,
      lateout("x1") _,
      lateout("x2") _,
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

#[cfg(target_pointer_width = "32")]
pub fn set_thread_priority(thread: Handle<Thread>, priority: u32) -> Result<(), ResultCode> {
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x0d",
      
      in("x0") thread.as_bits(),
      in("x1") priority,
      lateout("x0") error_code,
      lateout("x1") _,
      lateout("x2") _,
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