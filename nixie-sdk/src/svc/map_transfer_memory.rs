use crate::result::result_code::ResultCode;
use core::{arch::asm, ffi::c_void};
use super::{Handle, MemoryPermission, TransferMemory};

#[cfg(target_pointer_width = "64")]
pub fn map_transfer_memory(handle: Handle<TransferMemory>, address: *mut c_void, size: u64, permission: MemoryPermission) -> Result<(), ResultCode> {
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x51",
      
      in("x0") handle.as_bits(),
      in("x1") address,
      in("x2") size,
      in("x3") permission.0,
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