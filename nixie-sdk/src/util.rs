#[cfg(target_pointer_width = "64")]
pub type USIZE = zerocopy::native_endian::U64;

#[cfg(target_pointer_width = "32")]
pub type USIZE = zerocopy::native_endian::U32;