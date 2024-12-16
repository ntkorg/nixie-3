use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub fn notify(reason: u32, address: u64, size: u64) {
  unsafe {
    let updated_reason = reason | 0x80000000;
    asm!(
      "svc #0x26",
      
      in("x0") updated_reason,
      in("x1") address,
      in("x2") size,
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
pub fn notify(reason: u32, address: u64, size: u64) {
  unsafe {
    let updated_reason = reason | 0x80000000;
    asm!(
      "svc #0x26",
      
      in("w0") updated_reason,
      in("w1") address,
      in("w2") size,
      lateout("w0") _,
      lateout("w1") _,
      lateout("w2") _,
      lateout("w3") _,
    );
  }
}

pub fn abort(reason: u32, address: u64, size: u64) -> ! {
  unsafe {
    let updated_reason = reason & 0x7FFFFFFF;
  
    asm!(
      "svc #0x26",
      
      in("x0") updated_reason,
      in("x1") address,
      in("x2") size,
      options(noreturn)
    );
  }
}