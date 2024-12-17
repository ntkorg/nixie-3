use super::{CodeMemory, Handle, MemoryPermission, SlaveMemoryPermissions};
use crate::result::result_code::ResultCode;
use core::arch::asm;
use core::ffi::c_void;

#[cfg(target_pointer_width = "64")]
unsafe fn control_code_memory_raw(
  handle: Handle<CodeMemory>,
  operation: u32,
  address: *mut c_void,
  size: u64,
  permission: MemoryPermission,
) -> Result<(), ResultCode> {
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x4C",

      in("w0") handle.as_bits(),
      in("w1") operation,
      in("x2") address,
      in("x3") size,
      in("w4") permission.0,
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

pub unsafe fn map_owner(
  handle: Handle<CodeMemory>,
  address: *mut c_void,
  size: u64,
) -> Result<(), ResultCode> {
  control_code_memory_raw(
    handle,
    0,
    address,
    size,
    MemoryPermission::new().with_read(true).with_write(true),
  )
}

pub unsafe fn unmap_owner(
  handle: Handle<CodeMemory>,
  address: *mut c_void,
  size: u64,
) -> Result<(), ResultCode> {
  control_code_memory_raw(handle, 2, address, size, MemoryPermission::new())
}

pub unsafe fn map_slave(
  handle: Handle<CodeMemory>,
  address: *mut c_void,
  size: u64,
  permission: SlaveMemoryPermissions,
) -> Result<(), ResultCode> {
  control_code_memory_raw(
    handle,
    1,
    address,
    size,
    MemoryPermission::new()
      .with_read(true)
      .with_execute(permission == SlaveMemoryPermissions::ReadExecute),
  )
}

pub unsafe fn unmap_slave(
  handle: Handle<CodeMemory>,
  address: *mut c_void,
  size: u64,
) -> Result<(), ResultCode> {
  control_code_memory_raw(handle, 2, address, size, MemoryPermission::new())
}
