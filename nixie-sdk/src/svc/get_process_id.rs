use core::arch::asm;
use crate::result::result_code::ResultCode;
use super::{Handle, Process};

#[cfg(target_pointer_width = "64")]
pub fn get_process_id(process: Handle<Process>) -> Result<u64, ResultCode> {
  let mut error_code: u32;
  let mut process_id: u64;

  unsafe {
    asm!(
      "svc #0x24",
      
      in("w1") process.as_bits(),
      lateout("x0") error_code,
      lateout("x1") process_id,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(process_id);
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}
