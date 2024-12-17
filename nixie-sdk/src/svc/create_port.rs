use super::{ClientPort, Handle, ServerPort};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
fn create_port_raw(
  is_light: u32,
  name: u64,
  max_sessions: u32,
) -> Result<(Handle<ServerPort>, Handle<ClientPort>), ResultCode> {
  let mut error_code: usize;
  let mut server_port_bits: u32;
  let mut client_port_bits: u32;

  unsafe {
    asm!(
      "svc #0x70",

      in("w2") max_sessions,
      in("w3") is_light,
      in("x4") name,
      lateout("x0") error_code,
      lateout("x1") server_port_bits,
      lateout("x2") client_port_bits,
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
        Handle::<ServerPort>::from_bits(server_port_bits),
        Handle::<ClientPort>::from_bits(client_port_bits),
      )
    });
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}

pub fn create_light_port(
  name: heapless::String<8>,
  max_sessions: u32,
) -> Result<(Handle<ServerPort>, Handle<ClientPort>), ResultCode> {
  create_port_raw(
    1,
    u64::from_ne_bytes(name.as_bytes().try_into().unwrap()),
    max_sessions,
  )
}
pub fn create_port(
  name: heapless::String<8>,
  max_sessions: u32,
) -> Result<(Handle<ServerPort>, Handle<ClientPort>), ResultCode> {
  create_port_raw(
    0,
    u64::from_ne_bytes(name.as_bytes().try_into().unwrap()),
    max_sessions,
  )
}
