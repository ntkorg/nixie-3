use super::{ClientSession, Handle};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub fn connect_to_named_port(
  port_name: heapless::String<12>,
) -> Result<Handle<ClientSession>, ResultCode> {
  let mut error_code: u32;
  let mut client_session_handle: u32;

  unsafe {
    asm!(
      "svc #0x1F",

      in("x1") port_name.as_ptr(),
      lateout("w0") error_code,
      lateout("w1") client_session_handle,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(unsafe { Handle::<ClientSession>::from_bits(client_session_handle) });
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
