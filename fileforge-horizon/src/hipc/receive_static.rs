use fileforge_lib::{provider::r#trait::Provider, reader::{error::parse::ParseError, r#trait::readable::FixedSizeReadable, Reader}};
use zerocopy::FromBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Copy, Clone, Default, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct ReceiveStatic {
  address_low: u32,
  address_high: u16,
  size: u16,
}

impl ReceiveStatic {
  pub fn address(&self) -> u64 { ((self.address_high as u64) << 32) | (self.address_low as u64) }

  pub fn size(&self) -> u16 { self.size }

  pub fn set_address(&mut self, address: u64) {
    self.address_low = address as u32;
    self.address_high = (address >> 32) as u16;
  }

  pub fn set_size(&mut self, size: u16) { self.size = size; }

  pub fn with_address(mut self, address: u64) -> Self {
    self.set_address(address);
    self
  }

  pub fn with_size(mut self, size: u16) -> Self {
    self.set_size(size);
    self
  }

  pub fn new(address: u64, size: u16) -> Self {
    Self::default().with_address(address).with_size(size)
  }
}

pub enum ReceiveStaticReadError {}

impl<const NODE_NAME_SIZE: usize> fileforge_lib::error::Error<NODE_NAME_SIZE> for ReceiveStaticReadError {
  fn into_display(self) -> fileforge_lib::error::DisplayableError<NODE_NAME_SIZE, Self> where Self: Sized {
    unimplemented!()
  }

  fn with_report<Cb: FnMut(fileforge_lib::error::report::Report<NODE_NAME_SIZE>) -> ()>(&self, _: Cb) {
    unimplemented!()
  }
}

impl<'pool, const DIAGNOSTIC_NODE_NAME_SIZE: usize> FixedSizeReadable<'pool, DIAGNOSTIC_NODE_NAME_SIZE, 0x8> for ReceiveStatic {
  type Argument = ();
  type Error = ReceiveStaticReadError;

  fn read<RP: Provider>(reader: &mut Reader<'pool, DIAGNOSTIC_NODE_NAME_SIZE, RP>, _: Self::Argument) -> Result<Self, ParseError<'pool, Self::Error, RP::ReadError, DIAGNOSTIC_NODE_NAME_SIZE>> {
    Ok(reader.with_dyn_bytes(Some(0x8), "Contents", |bytes| {
      <Self as FromBytes>::read_from(bytes).unwrap()
    })?)
  }
}