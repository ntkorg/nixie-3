use core::arch::asm;

use zerocopy::{native_endian::{U16, U64}, FromBytes};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct LocalRegion {
  message_buffer: [u8; 256],
  disable_counter: U16,
  interrupt_flag: U16,
  reserved_0: [u8; 0x04],
  reserved_1: [u8; 0x78],
  tls: [u8; 0x50],
  locale_ptr: U64,
  errno_val: U64,
  thread_data: U64,
  eh_globals: U64,
  thread_pointer: U64,
  thread_type: U64,
}

pub (crate) fn get_local_region() -> *const [u8; 0x200] {
  let ptr: *const [u8; 0x200];
  
  unsafe {
    asm!("mrs {}, tpidrro_el0", out(reg) ptr);
  }

  ptr
}

pub (crate) fn get_mut_local_region() -> *mut [u8; 0x200] {
  let ptr: *mut [u8; 0x200];
  
  unsafe {
    asm!("mrs {}, tpidrro_el0", out(reg) ptr);
  }

  ptr
}