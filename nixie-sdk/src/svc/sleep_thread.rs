use core::arch::asm;

pub static YIELD_WITHOUT_CORE_MIGRATION: i64 = 0;
pub static YIELD_WITH_CORE_MIGRATION: i64 = -1;
pub static YIELD_TO_ANY_OTHER_THREAD: i64 = -2;

#[cfg(target_pointer_width = "64")]
pub fn sleep_thread(time: i64) -> () {
  unsafe {
    asm!(
      "svc #0x0b",

      in("x0") time,
      lateout("w0") _,
      lateout("w1") _,
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
pub unsafe fn sleep_thread(time: u64) -> () {
  let high: u32 = (time >> 32) as u32;
  let low: u32 = (time & 0xFFFFFFFF) as u32;

  unsafe {
    asm!(
      "svc #0x0b",

      in("w0") high,
      in("w1") low,
      lateout("w0") _,
      lateout("w1") _,
      lateout("w2") _,
      lateout("w3") _,
    );
  }
}
