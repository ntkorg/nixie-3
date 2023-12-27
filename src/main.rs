#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(naked_functions)]
#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::util::module::{get_global_ptr_mut, transmute_offset};
use crate::reloc::relocate_self;

mod reloc;
mod svc;
mod ipc;
mod util;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    svc::panic(2, 0, 0);
}

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
struct ModuleStart {
    branch_inst: u32,
    mod0_offset: u32,
}

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
struct Mod0 {
    magic: u32,
    dynamic_offset: u32,
    bss_start_offset: u32,
    bss_end_offset: u32,
    eh_start_offset: u32,
    eh_end_offset: u32,
    runtime_module_offset: u32,
}

#[no_mangle]
pub unsafe extern "C" fn startup(_x0: usize, _x1: usize) -> ! {

    let module_start = transmute_offset::<ModuleStart>(0);
    let mod0 = transmute_offset::<Mod0>(module_start.mod0_offset as usize);

    let bss_width = mod0.bss_end_offset - mod0.bss_start_offset;
    let bss_ptr = get_global_ptr_mut(mod0.bss_start_offset as usize);

    let bss_section = core::slice::from_raw_parts_mut(bss_ptr, bss_width as usize);

    bss_section.fill(0);

    relocate_self(mod0);

    svc::panic(3,2,1);

    // loop {}
}
global_asm!(r#"
.section .text.jmp, "ax", %progbits
.balign 4
.global beginning
beginning:
    b startup
	.word __module_header - beginning

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
