use crate::result::result_code::ResultCode;
use core::arch::asm;
use core::ffi::c_void;
use super::{Handle, Thread};

pub static DEFAULT_PROCESSOR: i32 = -2;

#[cfg(target_pointer_width = "64")]
pub unsafe fn create_thread(entry: extern "C" fn (*mut c_void) -> (), thread_argument: *mut c_void, stack_top: *mut c_void, priority: i32, processor_id: i32) -> Result<Handle<Thread>, ResultCode> {
  let mut handle_int: u32;
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x08",
      
      in("x1") entry,
      in("x2") thread_argument,
      in("x3") stack_top,
      in("w4") priority,
      in("w5") processor_id,
      lateout("w0") error_code,
      lateout("w1") handle_int,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(Handle::<Thread>::from_bits(handle_int));
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}

#[cfg(target_pointer_width = "32")]
pub unsafe fn create_thread(entry: extern "C" fn (*mut c_void) -> (), thread_argument: *mut c_void, stack_top: *mut c_void, priority: i32, processor_id: i32) -> Result<Handle<Thread>, ResultCode> {
  let mut handle_int: u32;
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x08",
      
      in("w1") entry,
      in("w2") thread_argument,
      in("w3") stack_top,
      in("w0") priority,
      in("w4") processor_id,
      lateout("w0") error_code,
      lateout("w1") handle_int,
      lateout("w2") _,
      lateout("w3") _,
    );
  }

  if error_code == 0 {
    return Ok(Handle::<Thread>::from_bits(handle_int));
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}