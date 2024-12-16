use core::{arch::asm, ffi::c_void};
use crate::result::result_code::ResultCode;
use super::{Handle, Process};

#[cfg(target_pointer_width = "64")]
pub unsafe fn map_process_code_memory(dest: *mut c_void, size: u64, process: Handle<Process>, process_src: *mut c_void) -> Result<(), ResultCode> {
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x77",
      
      in("x0") process.as_bits(),
      in("x1") dest,
      in("x2") process_src,
      in("x3") size,
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