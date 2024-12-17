use super::{Handle, ServerSession};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub unsafe fn reply_and_recieve_light(port: Handle<ServerSession>) -> Result<(), ResultCode> {
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x42",

      in("x1") port.as_bits(),
      lateout("w0") error_code,
      lateout("w1") _,
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
