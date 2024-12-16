use core::arch::asm;
use crate::result::result_code::ResultCode;

#[cfg(target_pointer_width = "64")]
pub fn unmap_physical_memory(address: *const u8, size: u64) -> Result<(), ResultCode> {
  let mut error_code: u32;

  unsafe {
    asm!(
      "svc #0x2D",
      
      in("x0") address as usize,
      in("x1") size,
      lateout("w0") error_code,
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
