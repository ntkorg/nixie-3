use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub unsafe fn get_current_processor_number() -> u32 {
  let mut cpuid: u32;

  unsafe {
    asm!(
      "svc #0x10",
      
      lateout("x0") cpuid,
      lateout("x1") _,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  return cpuid;
}

#[cfg(target_pointer_width = "32")]
pub unsafe fn get_current_processor_number() -> u32 {
  let mut cpuid: u32;

  unsafe {
    asm!(
      "svc #0x10",
      
      lateout("w0") cpuid,
      lateout("w1") _,
      lateout("w2") _,
      lateout("w3") _,
    );
  }

  return cpuid;
}