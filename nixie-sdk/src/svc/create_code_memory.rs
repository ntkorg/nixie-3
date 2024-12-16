use crate::result::result_code::ResultCode;
use core::arch::asm;
use super::{Handle, CodeMemory};
use core::ffi::c_void;

#[cfg(target_pointer_width = "64")]
pub unsafe fn create_code_memory(address: *mut c_void, size: u64) -> Result<Handle<CodeMemory>, ResultCode> {
  let mut error_code: u32;
  let mut handle_bits: u32;

  unsafe {
    asm!(
      "svc #0x4B",
      
      in("x1") address,
      in("x2") size,
      lateout("x0") error_code,
      lateout("x1") handle_bits,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(unsafe { Handle::<CodeMemory>::from_bits(handle_bits) });
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}