use core::arch::asm;

pub fn exit_process() -> ! {
  unsafe {
    asm!("svc #0x07", options(noreturn));
  }
}
