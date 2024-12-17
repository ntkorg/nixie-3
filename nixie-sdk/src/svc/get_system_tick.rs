use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub fn get_system_tick() -> u64 {
  let mut tick: u64;

  unsafe {
    asm!(
      "svc #0x1E",

      lateout("x0") tick,
      lateout("x1") _,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  return tick;
}
