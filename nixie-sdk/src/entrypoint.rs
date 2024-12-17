use core::arch::naked_asm;

use crate::{svc::Handle, thread::Thread};

extern "Rust" {
  fn main() -> ();
}

#[no_mangle]
extern "C" fn __nixie_entrypoint(x0: usize, x1: usize) {
  if x0 != 0 {
    // Exception
    crate::svc::r#break::abort(0x101000, 0, 0)
  }
  
  if !cfg!(feature = "nnsdk_sidecar") {
    let handle = unsafe { Handle::<crate::svc::Thread>::from_bits(x1 as u32) };

    unsafe { Thread::initialize_standalone_main_thread_from_handle(handle) };

    crate::svc::output_debug_string::output_debug_string("NixieSdk Initialized.");
  } else {
    unimplemented!("Initialize the NNSDK")
  }

  unsafe {
    main();
  }
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
