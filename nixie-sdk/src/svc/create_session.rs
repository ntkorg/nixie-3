use super::{ClientSession, Handle, ServerSession};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
fn create_session_raw(
  is_light: u32,
  name: u64,
) -> Result<(Handle<ServerSession>, Handle<ClientSession>), ResultCode> {
  let mut error_code: usize;
  let mut server_session_bits: u32;
  let mut client_session_bits: u32;

  unsafe {
    asm!(
      "svc #0x40",

      in("w2") is_light,
      in("w3") name,
      lateout("x0") error_code,
      lateout("x1") server_session_bits,
      lateout("x2") client_session_bits,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(unsafe {
      (
        Handle::<ServerSession>::from_bits(server_session_bits),
        Handle::<ClientSession>::from_bits(client_session_bits),
      )
    });
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}

pub fn create_light_session(
  name: heapless::String<8>,
) -> Result<(Handle<ServerSession>, Handle<ClientSession>), ResultCode> {
  create_session_raw(1, u64::from_ne_bytes(name.as_bytes().try_into().unwrap()))
}
pub fn create_session(
  name: heapless::String<8>,
) -> Result<(Handle<ServerSession>, Handle<ClientSession>), ResultCode> {
  create_session_raw(0, u64::from_ne_bytes(name.as_bytes().try_into().unwrap()))
}
