#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

use zerocopy::little_endian::U32;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::module::{get_global_ptr_mut, transmute_offset};
use crate::reloc::relocate_self;

mod reloc;
mod module;
mod svc;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
struct ModuleStart {
    branch_inst: U32,
    mod0_offset: U32,
}

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
struct Mod0 {
    magic: U32,
    dynamic_offset: U32,
    bss_start_offset: U32,
    bss_end_offset: U32,
    eh_start_offset: U32,
    eh_end_offset: U32,
    runtime_module_offset: U32,
}

#[no_mangle]
#[link_section = ".text.sss"]
pub unsafe extern "C" fn startup(_x0: usize, _x1: usize, module_start: usize) -> ! {

    let module_start = transmute_offset::<ModuleStart>(0);
    let mod0 = transmute_offset::<Mod0>(module_start.mod0_offset.get() as usize);

    let bss_width = mod0.bss_end_offset.get() - mod0.bss_start_offset.get();
    let bss_ptr = get_global_ptr_mut(mod0.bss_start_offset.get() as usize);

    let bss_section = core::slice::from_raw_parts_mut(bss_ptr, bss_width as usize);

    bss_section.fill(0);

    relocate_self()

    loop {}
}
global_asm!(r#"
.section .text.jmp, "ax", %progbits
.balign 4
beginning:
	b entrypoint
	.word __module_header - beginning

entrypoint:
    adrp x2, beginning
    b startup

.section .text, "ax", %progbits
.balign 4

.global __module_header
__module_header:
	.ascii "MOD0"
	.word __dynamic_start - __module_header
	.word __bss_start - __module_header
	.word __bss_end - __module_header
	.word __eh_frame_hdr_start - __module_header
	.word __eh_frame_hdr_end - __module_header
	.word 0 // Runtime-generated module object offset, unused
"#);