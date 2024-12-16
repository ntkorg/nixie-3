use crate::result::result_code::ResultCode;
use core::arch::asm;
use super::{Handle, MemoryPermission, SharedMemory};

#[cfg(target_pointer_width = "64")]
pub fn create_shared_memory(size: u64, local_memory_permission: MemoryPermission, remote_memory_permission: MemoryPermission) -> Result<Handle<SharedMemory>, ResultCode> {
  let mut error_code: u32;
  let mut handle_bits: u32;

  unsafe {
    asm!(
      "svc #0x50",
      
      in("x1") size,
      in("x2") local_memory_permission.0,
      in("x3") remote_memory_permission.0,
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
    return Ok(unsafe { Handle::<SharedMemory>::from_bits(handle_bits) });
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}