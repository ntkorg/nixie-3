use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub unsafe fn synchronize_preemption_state() -> () {
  unsafe {
    asm!(
      "svc #0x36",
      
      lateout("x0") _,
      lateout("x1") _,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }
}