use super::{Handle, Process};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub unsafe fn start_process(
  handle: Handle<Process>,
  main_thread_priority: i32,
  default_cpu: i32,
  main_thread_stack_size: u64,
) -> Result<(), ResultCode> {
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x7A",

      in("w0") handle.as_bits(),
      in("w1") main_thread_priority,
      in("w2") default_cpu,
      in("x3") main_thread_stack_size,
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
