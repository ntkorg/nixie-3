use core::arch::asm;
use crate::result::result_code::ResultCode;
use super::{ClientPort, ClientSession, Handle};

#[cfg(target_pointer_width = "64")]
pub fn connect_to_port(port: Handle<ClientPort>) -> Result<Handle<ClientSession>, ResultCode> {
  let mut error_code: usize;
  let mut session_bits: u32;

  unsafe {
    asm!(
      "svc #0x72",
      
      in("x1") port.as_bits(),
      lateout("x0") error_code,
      lateout("x1") session_bits,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(unsafe {
      Handle::<ClientSession>::from_bits(session_bits)
    });
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}