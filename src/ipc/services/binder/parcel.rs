use alloc::vec::Vec;
use zerocopy::AsBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::util::align::align_up;

#[derive(FromBytes, FromZeroes, AsBytes, Clone, Copy)]
#[repr(C, packed)]
pub struct ParcelHeader {
  pub payload_size: u32,
  pub payload_offset: u32,
  pub objects_size: u32,
  pub objects_offset: u32,
}

pub struct Parcel {
  data: Vec<u8>,
  offset: usize,
}

impl Parcel {
  fn align(&mut self) {
    let new_len = align_up(self.data.len(), 4);

    self.data.resize(new_len, 0);
  }

  fn write_buffer(&mut self, buffer: &[u8]) {
    self.data.extend_from_slice(buffer);

    self.align();
  }

  pub fn write<T: AsBytes>(&mut self, value: T) {
    self.write_buffer(value.as_bytes());
  }

  pub fn write_string(&mut self, string: &str) {
    self.write(string.encode_utf16().count() as u32);

    for c in string.encode_utf16() {
      self.data.extend_from_slice(&c.to_le_bytes());
    }

    self.align()
  }

  pub fn write_interface_token(&mut self, string: &str) {
    self.write(0x100u32);
    self.write_string(string)
  }
}
