use super::{Handle, MemoryPermission, Process};
use crate::result::result_code::ResultCode;
use core::{arch::asm, ffi::c_void};

#[cfg(target_pointer_width = "64")]
pub fn connect_to_port(
  port: Handle<Process>,
  address: *mut c_void,
  size: u64,
  memory_permission: MemoryPermission,
) -> Result<(), ResultCode> {
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x73",

      in("x0") port.as_bits(),
      in("x1") address,
      in("x2") size,
      in("x3") memory_permission.0,
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
