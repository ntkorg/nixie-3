use core::arch::asm;

#[naked]
pub fn r#break(reason: u32) -> ! {
    unsafe {
        asm!(
            "svc #0x26"
            options(noreturn)
        )
    }
}