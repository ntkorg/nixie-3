use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Copy, Clone, Default, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct DomainOutHeader {
  object_count: u32,
  padding: [u32; 3],
}

impl DomainOutHeader {
  pub fn object_count(&self) -> u32 { self.object_count }

  pub fn set_object_count(&mut self, object_count: u32) { self.object_count = object_count; }

  pub fn with_object_count(mut self, object_count: u32) -> Self {
    self.set_object_count(object_count);
    self
  }

  pub fn new(object_count: u32) -> Self { Self::default().with_object_count(object_count) }
}
