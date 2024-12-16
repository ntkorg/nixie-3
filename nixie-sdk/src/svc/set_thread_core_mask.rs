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

pub static DONT_UPDATE: i32 = -3;
pub static USE_PROCESS_VALUE: i32 = -2;
pub static DONT_CARE: i32 = -1;

#[cfg(target_pointer_width = "64")]
pub fn set_thread_core_mask(thread: Handle<Thread>, affinity_mask: u64, ideal_core: Option<i32>) -> Result<(), ResultCode> {
  let mut error_code: u32;
  let ideal_core: i32 = ideal_core.unwrap_or(DONT_UPDATE);

  unsafe {
    asm!(
      "svc #0x0f",
      
      in("x0") thread.as_bits(),
      in("x1") ideal_core,
      in("x2") affinity_mask,
      lateout("x0") error_code,
      lateout("x1") _,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(());
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}

#[cfg(target_pointer_width = "32")]
pub fn set_thread_core_mask(thread: Handle<Thread>, affinity_mask: u64, ideal_core: Option<i32>) -> Result<(), ResultCode> {
  let mut error_code: u32;
  let ideal_core: i32 = ideal_core.unwrap_or(DONT_UPDATE);
  let affinity_mask_high = (affinity_mask >> 32) as u32;
  let affinity_mask_low = (affinity_mask & 0xFFFFFFFF) as u32;

  unsafe {
    asm!(
      "svc #0x0f",
      
      in("w0") thread.as_bits(),
      in("w1") ideal_core,
      in("w2") affinity_mask_high,
      in("w3") affinity_mask_low,
      lateout("w0") error_code,
      lateout("w1") _,
      lateout("w2") _,
      lateout("w3") _,
    );
  }

  if error_code == 0 {
    return Ok(());
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}