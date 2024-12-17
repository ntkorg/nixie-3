use super::{Handle, ServerPort, ServerSession};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub unsafe fn accept_session(
  port: Handle<ServerPort>,
) -> Result<Handle<ServerSession>, ResultCode> {
  let mut handle_int: u32;
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x41",

      in("x1") port.as_bits(),
      lateout("w0") error_code,
      lateout("w1") handle_int,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(Handle::<ServerSession>::from_bits(handle_int));
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
