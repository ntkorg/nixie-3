use core::arch::asm;

use crate::result::result_code::ResultCode;

use super::{Handle, Process};

#[cfg(target_pointer_width = "64")]
fn get_process_info_raw(handle_bits: u32, info_type: u32) -> Result<u64, ResultCode> {
  let mut error_code: usize;
  let mut result: u64;

  unsafe {
    asm!(
      "svc #0x7C",

      in("w1") info_type,
      in("w2") handle_bits,
      lateout("x0") error_code,
      lateout("x1") result,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(result);
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}

pub fn get_process_state(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_process_info_raw(process.as_bits(), 0)
}
