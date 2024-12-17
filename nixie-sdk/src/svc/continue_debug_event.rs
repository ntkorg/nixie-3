use super::{ContinueDebugFlags, Debug, Handle};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub fn continue_debug_event(
  handle: Handle<Debug>,
  flags: ContinueDebugFlags,
  thread_ids: &[u64],
) -> Result<(), ResultCode> {
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x64",

      in("x0") handle.as_bits(),
      in("x1") flags.0,
      in("x2") thread_ids.as_ptr(),
      in("x3") thread_ids.len(),
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

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
