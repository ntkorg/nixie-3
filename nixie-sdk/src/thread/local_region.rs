use core::arch::asm;

use zerocopy::native_endian::U16;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct LocalRegion {
  pub(crate) message_buffer: [u8; 256],
  pub(crate) disable_counter: U16,
  pub(crate) interrupt_flag: U16,
  pub(crate) cache_maintenance_flag: u8,
  pub(crate) padding: [u8; 0x3],
  pub(crate) content: [u8; 0xF8],
}

impl !Send for LocalRegion {}

pub(crate) fn get_local_region() -> &'static LocalRegion {
  let ptr: *const [u8; 0x200];

  unsafe {
    asm!("mrs {}, tpidrro_el0", out(reg) ptr);

    &*(ptr as *const LocalRegion)
  }
}

pub(crate) fn get_mut_local_region() -> *mut LocalRegion {
  let ptr: *mut [u8; 0x200];

  unsafe {
    asm!("mrs {}, tpidrro_el0", out(reg) ptr);
  }

  ptr as *mut LocalRegion
}
