use core::arch::asm;
use super::{Handle, ClientSession, ReadableEvent};
use crate::result::result_code::ResultCode;

#[cfg(target_pointer_width = "64")]
pub fn send_async_request_with_user_buffer(session: &Handle<ClientSession>, buffer: &mut [u8]) -> Result<Handle<ReadableEvent>, ResultCode> {
  let mut error_code: u32;
  let mut readable_event_bits: u32;
  
  unsafe {
    asm!(
      "svc #0x23",
      
      in("x1") buffer.as_mut_ptr(),
      in("x2") buffer.len(),
      in("w3") session.as_bits(),
      lateout("w0") error_code,
      lateout("w1") readable_event_bits,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(unsafe { Handle::<ReadableEvent>::from_bits(readable_event_bits) });
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}
