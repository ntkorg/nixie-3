use super::{Handle, ServerPort};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub fn manage_named_port(
  name: &heapless::String<12>,
  max_sessions: u32,
) -> Result<Handle<ServerPort>, ResultCode> {
  let mut error_code: usize;
  let mut server_port_bits: u32;

  unsafe {
    asm!(
      "svc #0x71",

      in("x1") name.as_ptr(),
      in("w2") max_sessions,
      lateout("x0") error_code,
      lateout("x1") server_port_bits,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(unsafe { Handle::<ServerPort>::from_bits(server_port_bits) });
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
