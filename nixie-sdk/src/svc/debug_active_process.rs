use core::arch::asm;
use super::{Handle, Debug};
use crate::result::result_code::ResultCode;

#[cfg(target_pointer_width = "64")]
pub fn debug_active_process(process_id: u64) -> Result<Handle<Debug>, ResultCode> {
  let mut error_code: u32;
  let mut debug_bits: u32;
  
  unsafe {
    asm!(
      "svc #0x60",
      
      in("x0") process_id,
      lateout("w0") error_code,
      lateout("w1") debug_bits,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(unsafe { Handle::<Debug>::from_bits(debug_bits) });
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}
