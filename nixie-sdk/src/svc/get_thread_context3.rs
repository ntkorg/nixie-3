use core::arch::asm;
use zerocopy::FromZeroes;

use super::{Handle, Thread, ThreadContext};

#[cfg(target_pointer_width = "64")]
pub unsafe fn get_thread_context3(
  thread: Handle<Thread>,
) -> Result<ThreadContext, crate::result::result_code::ResultCode> {
  let mut thread_context = ThreadContext::new_zeroed();
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x33",

      in("x0") &mut thread_context as *mut ThreadContext as usize,
      in("w1") thread.as_bits(),
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
    return Ok(thread_context);
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
