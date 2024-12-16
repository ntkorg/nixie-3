use core::arch::asm;

pub fn exit_thread() -> ! {
  unsafe {
    asm!(
      "svc #0x0a",
      options(noreturn)
    );
  }
}