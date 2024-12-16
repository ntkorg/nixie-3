use core::arch::asm;
use crate::result::result_code::ResultCode;
use super::{Handle, ReadableEvent, WritableEvent};

#[cfg(target_pointer_width = "64")]
pub fn create_event() -> Result<(Handle<WritableEvent>, Handle<ReadableEvent>), ResultCode> {
  let mut error_code: u32;
  let mut writable_event_handle: u32;
  let mut readable_event_handle: u32;

  unsafe {
    asm!(
      "svc #0x45",
      
      lateout("w0") error_code,
      lateout("w1") writable_event_handle,
      lateout("w2") readable_event_handle,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(unsafe{(
      Handle::<WritableEvent>::from_bits(writable_event_handle),
      Handle::<ReadableEvent>::from_bits(readable_event_handle),
    )});
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}