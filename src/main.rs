#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(panic_info_message)]
#![feature(generic_const_exprs)]
#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

use ipc::services::ServiceRoot;
use ipc::services::am::ApplicationManagerOE;
use ipc::services::lm::LogManager;
use ipc::services::sm::{ServiceManager, ServiceName};
use ipc::sf::Error;
use util::{magic::reverse_magic, tls};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::reloc::relocate_self;
use crate::util::module::{get_global_ptr_mut, transmute_offset};

mod ipc;
mod reloc;
mod svc;
mod util;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    info.message().map(|m| {
        m.as_str().map(|s| {
            let bytes = s.as_bytes();
            _ = tls::get_writer(0, bytes.len()).write_vec(bytes);
            svc::panic(1, s.as_ptr() as usize, s.len());
        });

        svc::panic(3, 0, 0);
    });
    svc::panic(2, 1, 0);
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

    let sm =
        ServiceManager::connect().unwrap_or_else(|res| svc::panic(10, res.value() as usize, 0));
    if let Err(err) = sm.register_client() {
        match err {
            Error::InvalidRequest(_) => svc::panic(11, 0, 0),
            Error::InvalidResponse(_) => svc::panic(12, 0, 0),
            Error::RequestError(res) => svc::panic(13, res.value() as usize, 0),
            Error::ResponseError(res) => svc::panic(14, res.value() as usize, 0),
            Error::NoMoveHandle => svc::panic(15, 0, 0),
            Error::NoSendHandle => svc::panic(16, 0, 0),
            Error::NoObject => svc::panic(17, 0, 0),
            Error::NoPid => svc::panic(18, 0, 0),
            Error::NoStatic => svc::panic(19, 0, 0),
            Error::NotEnoughData => svc::panic(20, 0, 0),
            Error::InvalidData => svc::panic(21, 0, 0),
        }
    }

    // let am = ApplicationManagerOE::open(&sm).unwrap_or_else(|_| svc::panic(30, 0, 0));
    // let ap = am.open_application_proxy().unwrap_or_else(|err| match err {
    //     Error::InvalidRequest(_) => svc::panic(31, 0, 0),
    //     Error::InvalidResponse(_) => svc::panic(32, 0, 0),
    //     Error::RequestError(res) => svc::panic(33, res.value() as usize, 0),
    //     Error::ResponseError(res) => svc::panic(34, res.value() as usize, 0),
    //     Error::NoMoveHandle => svc::panic(35, 0, 0),
    //     Error::NoSendHandle => svc::panic(36, 0, 0),
    //     Error::NoObject => svc::panic(37, 0, 0),
    //     Error::NoPid => svc::panic(38, 0, 0),
    //     Error::NoStatic => svc::panic(39, 0, 0),
    //     Error::NotEnoughData => svc::panic(40, 0, 0),
    //     Error::InvalidData => svc::panic(41, 0, 0),
    // });
    svc::panic(3, 2, 1);

    // loop {}
}
global_asm!(
    r#"
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
"#
);
