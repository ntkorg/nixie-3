use core::{arch::naked_asm, fmt::Write};

use crate::svc::{Handle, Thread};

extern "Rust" {
  fn main() -> ();
}

#[no_mangle]
extern "C" fn __nixie_entrypoint(x0: usize, x1: usize) {
  if x0 != 0 {
    // Exception
    crate::svc::r#break::abort(0x101000, 0, 0)
  }

  let handle = unsafe { Handle::<Thread>::from_bits(x1 as u32) };
  let current_local_region = crate::thread::local_region::get_mut_local_region();

  crate::svc::output_debug_string::output_debug_string("NixieSdk Initialized.");

  unsafe { main(); }
}

#[naked]
#[no_mangle]
unsafe extern "C" fn __nixie_module_header() {
  naked_asm!(
    r#".ascii "MOD0""#,
    ".word __dynamic_start - __nixie_module_header",
    ".word __bss_start - __nixie_module_header",
    ".word __bss_end - __nixie_module_header",
    ".word __eh_frame_hdr_start - __nixie_module_header",
    ".word __eh_frame_hdr_end - __nixie_module_header",
    ".word 0",
  );
}

#[naked]
#[link_section = ".text.jmp"]
#[no_mangle]
unsafe extern "C" fn __nixie_module_intro() -> ! {
  naked_asm!(
    "b __nixie_entrypoint",
    ".word __nixie_module_header - __nixie_module_intro",
  )
}