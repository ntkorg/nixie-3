use crate::result::result_code::ResultCode;
use core::arch::asm;
use core::ffi::c_void;

#[cfg(target_pointer_width = "64")]
pub unsafe fn set_memory_attribute(pointer: *mut c_void, size: usize, mask: u32, value: u32) -> Result<(), ResultCode> {
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x03",
      
      in("x0") pointer,
      in("x1") size,
      in("w2") mask,
      in("w3") value,
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
pub unsafe fn set_memory_attribute(pointer: *mut c_void, size: usize, mask: u32, value: u32) -> Result<(), ResultCode> {
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x03",
      
      in("w0") pointer,
      in("w1") size,
      in("w2") mask,
      in("w3") value,
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