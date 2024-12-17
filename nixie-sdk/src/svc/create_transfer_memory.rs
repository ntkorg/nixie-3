use super::{Handle, MemoryPermission, TransferMemory};
use crate::result::result_code::ResultCode;
use core::arch::asm;
use core::ffi::c_void;

#[cfg(target_pointer_width = "64")]
pub unsafe fn create_transfer_memory(
  address: *mut c_void,
  size: u64,
  permissions: MemoryPermission,
) -> Result<Handle<TransferMemory>, ResultCode> {
  let mut error_code: usize;
  let mut handle_int: u32;

  unsafe {
    asm!(
      "svc #0x15",

      in("x1") address,
      in("x2") size,
      in("w3") permissions.into_bits(),
      lateout("x0") error_code,
      lateout("x1") handle_int,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(Handle::<TransferMemory>::from_bits(handle_int));
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
