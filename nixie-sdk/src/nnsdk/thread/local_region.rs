use zerocopy::native_endian::U64;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(FromBytes, AsBytes, FromZeroes)]
#[repr(C)]
pub struct NnsdkLocalRegion {
  reserved_1: [u8; 0x78],
  tls: [u8; 0x50],
  locale_ptr: U64,
  errno_val: U64,
  thread_data: U64,
  eh_globals: U64,
  thread_pointer: U64,
  thread_type: U64,
}