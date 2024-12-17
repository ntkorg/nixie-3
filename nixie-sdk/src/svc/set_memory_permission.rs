use super::MemoryPermission;
use crate::result::result_code::ResultCode;
use core::arch::asm;
use core::ffi::c_void;

#[cfg(target_pointer_width = "64")]
pub unsafe fn set_memory_permission(
  pointer: *mut c_void,
  size: usize,
  permission: MemoryPermission,
) -> Result<(), ResultCode> {
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x02",

      in("x0") pointer,
      in("x1") size,
      in("w2") permission.into_bits(),
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

#[cfg(target_pointer_width = "32")]
pub unsafe fn set_memory_permission(
  pointer: *mut c_void,
  size: usize,
  permission: MemoryPermission,
) -> Result<(), ResultCode> {
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x02",

      in("w0") pointer,
      in("w1") size,
      in("w2") permission.into_bits(),
      lateout("w0") error_code,
      lateout("w1") _,
      lateout("w2") _,
      lateout("w3") _,
    );
  }

  if error_code == 0 {
    return Ok(());
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
