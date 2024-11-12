#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(panic_info_message)]
#![feature(generic_const_exprs)]
#![feature(gen_blocks)]
#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use core::arch::{asm, global_asm};
use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::write_bytes;
use core::time::Duration;

use heapless::String;
use ipc::services::am::ApplicationManagerOE;
use ipc::services::lm::LogManager;
use ipc::services::sm::{ServiceManager, ServiceName};
use ipc::services::ServiceRoot;
use ipc::sf::Error;
use svc::{output_debug_string, sleep_thread};
use util::{allocator, tls};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::ipc::services::binder::Binder;
use crate::reloc::relocate_self;
use crate::svc::process::{self, capability, CreateProcessFlags, CreateProcessParams};
use crate::svc::{Handle, Process};
use crate::util::magic::Magic;
use crate::util::module::{get_global_ptr_mut, transmute_offset};

mod ipc;
mod reloc;
mod svc;
mod util;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
  tls::dump();

  output_debug_string("Panic!");
  if !allocator::is_initialized() {
    output_debug_string("not initialized, panicked at");
    if let Some(location) = info.location() {
      output_debug_string(location.file());
    }
  };

  let mut str: String<0x2000> = String::new();
  write!(&mut str, "{}", info).unwrap();
  output_debug_string(str.as_str());
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
  let module_start = unsafe { transmute_offset::<ModuleStart>(0) };
  let mod0 = unsafe { transmute_offset::<Mod0>(module_start.mod0_offset as usize) };
  let mut log: String<4096> = String::new();

  let bss_width = mod0.bss_end_offset - mod0.bss_start_offset;
  let bss_ptr =
    unsafe { get_global_ptr_mut((module_start.mod0_offset + mod0.bss_start_offset) as usize) };

  {
    let mut writer = tls::get_writer(0, 0xFF);
    writer.write(bss_width).unwrap();
    writer
      .write((module_start.mod0_offset + mod0.bss_start_offset) as usize)
      .unwrap();
  }
  output_debug_string("hey look");
  // tls::dump();

  // write_bytes(bss_ptr, 0, bss_width as _);

  output_debug_string("in god we rust");

  unsafe { relocate_self(module_start.mod0_offset + mod0.dynamic_offset) }

  allocator::initialize(0x200000).unwrap();

  let sm = ServiceManager::connect().unwrap();
  sm.register_client().unwrap();

  fn write_text<F: FnOnce(&mut dyn Write) -> core::fmt::Result>(closure: F) {
    let mut str: String<0x2000> = String::new();
    closure(&mut str).unwrap();
    output_debug_string(str.as_str());
  }

  // let create_params = CreateProcessParams {
  //   name: b"svctest".try_into().unwrap(),
  //   process_category: 1,
  //   code_addr: 128 * 1024 * 1024,
  //   code_num_pages: 1,
  //   title_id: 0x0100000010101010,
  //   flags: CreateProcessFlags::new()
  //     .with_instruction_set(svc::process::InstructionSet::Aarch32)
  //     .with_enable_debug(true)
  //     .with_address_space(3)
  //     .with_is_app(true),
  //   system_resource_num_pages: 0,
  // };

  // let proc: Handle<Process> = Handle::<Process>::create(create_params, &[
  //   capability::program_type(capability::ApplicationType::Application),
  //   capability::debug_flags(true, true),
  //   capability::kernel_version(9, 0),
  //   capability::thread_info(0..=63, 0..=63),
  //   capability::handle_table_size(512),
  // ]).unwrap();

  // proc.terminate().unwrap();

  let vi = ipc::services::vi::ViUser::open(&sm).unwrap();

  let display_service = vi.get_display_service().unwrap();

  let display = display_service
  .open_display("Default".try_into().unwrap())
  .unwrap();

  write_text(|s| s.write_fmt(format_args!("Display: {display:?}")));

  let layer = display_service.create_stray_layer(&display, None).unwrap();

  let binder = Binder::from_layer(&layer, &display_service).unwrap();


  loop {
    sleep_thread(Duration::from_secs(1))
  }
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
