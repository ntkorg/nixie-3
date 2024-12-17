use crate::result::result_code::ResultCode;
use core::arch::asm;
use core::ffi::c_void;

#[cfg(target_pointer_width = "64")]
pub unsafe fn set_heap_size(size: usize) -> Result<*mut c_void, ResultCode> {
  let mut error_code: usize;
  let mut address: usize;

  unsafe {
    asm!(
      "svc #0x01",

      in("x1") size,
      lateout("x0") error_code,
      lateout("x1") address,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(address as *mut c_void);
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}

#[cfg(target_pointer_width = "32")]
pub unsafe fn set_heap_size(size: usize) {
  let mut error_code: usize;
  let mut address: usize;

  unsafe {
    asm!(
      "svc #0x01",

      in("w1") size,
      lateout("w0") error_code,
      lateout("w1") address,
      lateout("w2") _,
      lateout("w3") _,
    );
  }

  if error_code == 0 {
    return Ok(address as *mut c_void);
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
