use crate::result::result_code::ResultCode;
use core::{arch::asm, ffi::c_void};
use super::PhysicalMemoryInfo;

#[cfg(target_pointer_width = "64")]
pub fn query_physical_address(address: *mut c_void) -> Result<PhysicalMemoryInfo, ResultCode> {
  let mut error_code: u32;
  let mut physical_memory_info_address: u64;
  let mut physical_memory_info_base_address: u64;
  let mut physical_memory_info_size: u64;

  unsafe {
    asm!(
      "svc #0x54",
      
      in("x1") address,
      lateout("x0") error_code,
      lateout("x1") physical_memory_info_address,
      lateout("x2") physical_memory_info_base_address,
      lateout("x3") physical_memory_info_size,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(PhysicalMemoryInfo {
      address: physical_memory_info_address,
      base_address: physical_memory_info_base_address,
      size: physical_memory_info_size,
    });
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}