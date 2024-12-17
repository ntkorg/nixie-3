use super::MemoryInfo;
use crate::result::result_code::ResultCode;
use crate::svc::RawMemoryInfo;
use core::arch::asm;
use core::ffi::c_void;
use zerocopy::FromZeroes;

#[cfg(target_pointer_width = "64")]
pub fn query_memory(address: *mut c_void) -> Result<(MemoryInfo, u32), ResultCode> {
  let mut memory_info = RawMemoryInfo::new_zeroed();
  let mut error_code: u32;
  let mut page_flags: u32;

  unsafe {
    asm!(
      "svc #0x06",

      in("x0") &mut memory_info as *mut RawMemoryInfo,
      in("x2") address,
      lateout("x0") error_code,
      lateout("x1") page_flags,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok((MemoryInfo::from_raw(memory_info), page_flags));
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}

#[cfg(target_pointer_width = "32")]
pub fn query_memory(address: *mut c_void) -> Result<(MemoryInfo, u32), ResultCode> {
  let mut memory_info = RawMemoryInfo::new_zeroed();
  let mut error_code: u32;
  let mut page_flags: u32;

  unsafe {
    asm!(
      "svc #0x06",

      in("w0") &mut memory_info as *mut RawMemoryInfo,
      in("w2") address,
      lateout("w0") error_code,
      lateout("w1") page_flags,
      lateout("w2") _,
      lateout("w3") _,
    );
  }

  if error_code == 0 {
    return Ok((MemoryInfo::from_raw(memory_info), page_flags));
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
