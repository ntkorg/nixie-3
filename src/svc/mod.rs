mod handle;
pub mod process;
mod session;
mod thread;

pub use handle::Handle;
pub use process::Process;
pub use session::Session;
pub use thread::Thread;

use core::{any::Any, arch::asm, default, ffi::CStr, ops::Deref};

use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::util::result::ResultCode;

pub fn set_heap_size<'a>(size: usize) -> Result<&'a mut [u8], ResultCode> {
  let mut result;
  let mut ptr;
  unsafe {
    asm!(
        "svc #0x01",
        in("x1") size,
        lateout("w0") result,
        lateout("x1") ptr,
    );
  }
  ResultCode::as_result(result).map(|_| unsafe { core::slice::from_raw_parts_mut(ptr, size) })
}

#[derive(Copy, Clone, Default)]
pub enum MemoryPermission {
  #[default]
  None,
  Read,
  Write,
  Execute,
  ReadWrite,
  ReadExecute,
  WriteExecute,
  ReadWriteExecute,
}

impl MemoryPermission {
  const fn as_u32(&self) -> u32 {
    match self {
      MemoryPermission::None => 0,
      MemoryPermission::Read => 0b001,
      MemoryPermission::Write => 0b010,
      MemoryPermission::Execute => 0b100,
      MemoryPermission::ReadWrite => 0b011,
      MemoryPermission::ReadExecute => 0b101,
      MemoryPermission::WriteExecute => 0b110,
      MemoryPermission::ReadWriteExecute => 0b111,
    }
  }

  const fn from_u32(value: u32) -> Self {
    match value {
      0 => MemoryPermission::None,
      0b001 => MemoryPermission::Read,
      0b010 => MemoryPermission::Write,
      0b100 => MemoryPermission::Execute,
      0b011 => MemoryPermission::ReadWrite,
      0b101 => MemoryPermission::ReadExecute,
      0b110 => MemoryPermission::WriteExecute,
      0b111 => MemoryPermission::ReadWriteExecute,
      _ => panic!("Invalid memory permission value!"),
    }
  }
}

pub fn set_memory_permission(
  address: usize,
  size: usize,
  permission: MemoryPermission,
) -> Result<(), ResultCode> {
  let mut result;
  unsafe {
    asm!(
        "svc #0x02",
        in("x0") address,
        in("x1") size,
        in("x2") permission.as_u32(),
        lateout("w0") result,
    );
  }
  ResultCode::as_result(result)
}

pub fn set_memory_uncached(address: usize, size: usize, cached: bool) -> Result<(), ResultCode> {
  let mut result;
  unsafe {
    asm!(
        "svc #0x03",
        in("x0") address,
        in("x1") size,
        in("w2") 0x8,
        in("w3") if cached { 1 } else { 0 },
        lateout("w0") result,
    );
  }
  ResultCode::as_result(result)
}

pub fn map_memory(
  dest_address: usize,
  src_address: usize,
  size: usize,
  permission: MemoryPermission,
) -> Result<(), ResultCode> {
  let mut result;
  unsafe {
    asm!(
        "svc #0x04",
        in("x0") dest_address,
        in("x1") src_address,
        in("x2") size,
        in("w3") permission.as_u32(),
        lateout("w0") result,
    );
  }
  ResultCode::as_result(result)
}

pub fn unmap_memory(
  dest_address: usize,
  src_address: usize,
  size: usize,
) -> Result<(), ResultCode> {
  let mut result;
  unsafe {
    asm!(
        "svc #0x05",
        in("x0") dest_address,
        in("x1") src_address,
        in("x2") size,
        lateout("w0") result,
    );
  }
  ResultCode::as_result(result)
}

#[derive(Copy, Clone, Default, FromBytes, FromZeroes, AsBytes)]
#[repr(C, packed)]
pub struct MemoryInfo {
  address: u64,
  size: u64,
  r#type: u32,
  attribute: u32,
  permission: u32,
  device_ref_count: u32,
  ipc_ref_count: u32,
}

pub fn query_memory(address: usize) -> Result<MemoryInfo, ResultCode> {
  let mut result;
  let mut info = MemoryInfo::default();
  unsafe {
    asm!(
        "svc #0x06",
        in("x0") (&mut info) as *mut MemoryInfo,
        in("x1") address,
        lateout("w0") result,
    );
  }
  ResultCode::as_result(result).map(|_| info)
}

pub fn exit_process() -> ! {
  unsafe {
    asm!("svc #0x7", ".word 0", options(noreturn));
  }
}

pub fn panic(reason: u32, address: usize, size: usize) -> ! {
  // todo: notification only not supported, make a function for it
  unsafe {
    asm!(
        "svc #0x26",
        ".word 0",
        in("w0") reason,
        in("x1") address,
        in("x2") size,
        options(noreturn)
    );
  }
}

pub fn exit_thread() -> ! {
  unsafe {
    asm!("svc #0xA", ".word 0", options(noreturn));
  }
}

pub fn sleep_thread(timeout: core::time::Duration) {
  if timeout.as_nanos() > 0xFFFFFFFF {
    panic!("Timeout too large!");
  }
  unsafe {
    asm!(
        "svc #0x0B",
        in("x0") timeout.as_nanos() as u64,
    );
  }
}

pub fn output_debug_string(string: &str) {
  unsafe {
    asm!(
        "svc #0x27",
        in("x0") string.as_ptr(),
        in("x1") string.len(),
    )
  }
}

pub fn output_debug_bytes(string: &[u8]) {
  unsafe {
    asm!(
      "svc #0x27",
      in("x0") string.as_ptr(),
      in("x1") string.len(),
    )
  }
}
