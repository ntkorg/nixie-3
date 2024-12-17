use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub fn get_process_list(thread_ids: &mut [u64]) -> Result<u64, ResultCode> {
  let mut error_code: u32;
  let mut process_count: u64;

  unsafe {
    asm!(
      "svc #0x65",

      in("x1") thread_ids.as_ptr(),
      in("x2") thread_ids.len(),
      lateout("x0") error_code,
      lateout("x1") process_count,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(process_count);
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
