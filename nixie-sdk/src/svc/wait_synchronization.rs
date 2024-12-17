use super::{Handle, Waitable};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub fn wait_synchronization<T: Waitable>(
  handles: &[Handle<T>],
  timeout: u64,
) -> Result<&Handle<T>, ResultCode> {
  let mut error_code: usize;
  let mut handle_index: u64;

  unsafe {
    asm!(
      "svc #0x18",

      in("x1") handles.as_ptr(),
      in("w2") handles.len() as u32,
      in("x3") timeout,
      lateout("x0") error_code,
      lateout("x1") handle_index,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(&handles[handle_index as usize]);
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
