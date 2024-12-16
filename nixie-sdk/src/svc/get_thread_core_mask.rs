use crate::result::result_code::ResultCode;
use core::arch::asm;
use super::{Handle, Thread};

// core id - int32
// ideal core
// magic -3 => DONT_UPDATE
//   - persists the previous value
// magic -2 => USE_PROCESS_VALUE
// magic -1 => DONT_CARE

// affinity mask - int64
// what cores the thread *can* run on

#[cfg(target_pointer_width = "64")]
pub fn get_thread_core_mask(thread: Handle<Thread>) -> Result<(i32, u64), ResultCode> {
  let mut error_code: u32;
  let mut ideal_core: i32;
  let mut affinity_mask: u64;

  unsafe {
    asm!(
      "svc #0x0e",
      
      in("x0") thread.as_bits(),
      lateout("x0") error_code,
      lateout("x1") ideal_core,
      lateout("x2") affinity_mask,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok((ideal_core, affinity_mask));
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}

#[cfg(target_pointer_width = "32")]
pub fn get_thread_core_mask(thread: Handle<Thread>) -> Result<(i32, u64), ResultCode> {
  let mut error_code: u32;
  let mut ideal_core: i32;
  let mut affinity_mask_high: u32;
  let mut affinity_mask_low: u32;

  unsafe {
    asm!(
      "svc #0x0e",
      
      in("w0") thread.as_bits(),
      lateout("w0") error_code,
      lateout("w1") ideal_core,
      lateout("w2") affinity_mask_high,
      lateout("w3") affinity_mask_low,
      lateout("w4") _,
      lateout("w5") _,
      lateout("w6") _,
      lateout("w7") _,
    );
  }

  let affinity_mask = (affinity_mask_high << 32) | affinity_mask_low;

  if error_code == 0 {
    return Ok((ideal_core, affinity_mask));
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}