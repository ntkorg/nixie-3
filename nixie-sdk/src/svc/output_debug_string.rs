use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub fn output_debug_string(message: &str) {
  unsafe {
    let ptr = message.as_ptr();
    let length = message.len();

    asm!(
      "svc #0x27",

      in("x0") ptr,
      in("x1") length,
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
pub fn output_debug_string(message: &str) {
  unsafe {
    let ptr = message.as_ptr();
    let length = message.len();

    asm!(
      "svc #0x27",

      in("w0") ptr,
      in("w1") length,
      lateout("w0") _,
      lateout("w1") _,
      lateout("w2") _,
      lateout("w3") _,
    );
  }
}
