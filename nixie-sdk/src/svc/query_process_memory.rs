use crate::result::result_code::ResultCode;
use core::arch::asm;
use core::ffi::c_void;
use crate::svc::RawMemoryInfo;
use zerocopy::FromZeroes;
use super::{MemoryInfo, Handle, Process};

#[cfg(target_pointer_width = "64")]
pub fn query_process_memory(process: Handle<Process>, address: *mut c_void) -> Result<(MemoryInfo, u32), ResultCode> {
  let mut memory_info = RawMemoryInfo::new_zeroed();
  let mut error_code: u32;
  let mut page_flags: u32;

  unsafe {
    asm!(
      "svc #0x76",
      
      in("x0") &mut memory_info as *mut RawMemoryInfo,
      in("x2") process.as_bits(),
      in("x3") address,
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

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}