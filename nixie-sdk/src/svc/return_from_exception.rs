use core::arch::asm;

pub fn break_from_exception() -> ! {
  unsafe {
    asm!(
      "svc #0x28",
      
      in("w1") 1,
      options(noreturn),
    );
  }
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn continue_from_exception() -> () {
  asm!(
    "svc #0x28",
    
    in("w1") 0,
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

#[cfg(target_pointer_width = "32")]
pub unsafe fn continue_from_exception() -> () {
  asm!(
    "svc #0x28",
    
    in("w1") 0,
    lateout("w0") _,
    lateout("w1") _,
    lateout("w2") _,
    lateout("w3") _,
  );
}