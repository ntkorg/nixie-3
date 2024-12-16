use core::arch::asm;
use super::{Handle, Thread};

#[cfg(target_pointer_width = "64")]
pub unsafe fn start_thread(thread: Handle<Thread>) {
  unsafe {
    asm!(
      "svc #0x09",
      
      in("x0") thread.as_bits(),
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

#[cfg(target_pointer_width = "32")]
pub unsafe fn start_thread(thread: Handle<Thread>) {
  unsafe {
    asm!(
      "svc #0x09",
      
      in("w0") thread.as_bits(),
      lateout("w0") _,
      lateout("w1") _,
      lateout("w2") _,
      lateout("w3") _,
    );
  }
}