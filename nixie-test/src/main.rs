#![no_main]
#![no_std]
#![feature(panic_info_message)]

mod reloc;

use core::{fmt::Write, panic::PanicInfo};

use heapless::String;
use reloc::{get_global_ptr_mut, relocate_self, transmute_offset, Mod0};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
struct ModuleStart {
  branch_inst: u32,
  mod0_offset: u32,
}

#[no_mangle]
pub fn main() {
  let module_start = unsafe { transmute_offset::<ModuleStart>(0) };
  let mod0 = unsafe { transmute_offset::<Mod0>(module_start.mod0_offset as usize) };

  let bss_width = mod0.bss_end_offset - mod0.bss_start_offset;
  let bss_ptr = unsafe { get_global_ptr_mut(mod0.bss_start_offset as usize) };

  let bss_section = unsafe { core::slice::from_raw_parts_mut(bss_ptr, bss_width as usize) };

  bss_section.fill(0);

  unsafe { relocate_self(module_start.mod0_offset + mod0.dynamic_offset) };

  panic!("Hello, world!");
}

#[panic_handler]
pub fn panic_handler(panic_info: &PanicInfo) -> ! {
  nixie_sdk::svc::output_debug_string::output_debug_string(" --- Rust Panic --- ");

  let message = panic_info.message();
  let mut log: String<4096> = String::new();
  let _ = log.write_str("message: ");
  let _ = log.write_fmt(format_args!("{}", message));
  nixie_sdk::svc::output_debug_string::output_debug_string(&log);

  if let Some(location) = panic_info.location() {
    nixie_sdk::svc::output_debug_string::output_debug_string("");
    let mut log: String<4096> = String::new();
    let _ = log.write_fmt(format_args!("at: {}:{}:{}", location.file(), location.line(), location.column()));
    nixie_sdk::svc::output_debug_string::output_debug_string(&log);
  }

  nixie_sdk::svc::r#break::abort(0, 0, 0);
}