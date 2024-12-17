use super::{Handle, Process};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub unsafe fn terminate_process(handle: Handle<Process>) -> Result<(), ResultCode> {
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x7B",

      in("w0") handle.as_bits(),
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
