use core::arch::asm;
use super::{Handle, Process, ProcessActivity};

#[cfg(target_pointer_width = "64")]
pub unsafe fn set_process_activity(process: Handle<Process>, activity: ProcessActivity) -> Result<(), crate::result::result_code::ResultCode> {
  let mut error_code: usize;

  unsafe {
    asm!(
      "svc #0x4F",
      
      in("w0") process.as_bits(),
      in("w1") activity as u32,
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