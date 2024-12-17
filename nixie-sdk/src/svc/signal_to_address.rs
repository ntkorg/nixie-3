use super::ArbitrationType;
use core::{arch::asm, ffi::c_void};

#[cfg(target_pointer_width = "64")]
pub unsafe fn signal_to_address(
  address: *const c_void,
  arbitration_type: ArbitrationType,
  value: u32,
  num_to_signal: u32,
) -> Result<(), crate::result::result_code::ResultCode> {
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x35",

      in("x0") address as usize,
      in("w1") arbitration_type as u32,
      in("w2") value,
      in("w3") num_to_signal,
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

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
