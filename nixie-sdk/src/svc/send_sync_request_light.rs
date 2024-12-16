use core::arch::asm;
use super::{Handle, ClientSession};
use crate::result::result_code::ResultCode;

#[cfg(target_pointer_width = "64")]
pub fn send_sync_request_light(session: Handle<ClientSession>) -> Result<(), ResultCode> {
  let mut error_code: u32;
  
  unsafe {
    asm!(
      "svc #0x20",
      
      in("x1") session.as_bits(),
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

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}
