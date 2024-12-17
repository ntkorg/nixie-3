use fileforge_lib::{provider::r#trait::Provider, reader::{error::parse::ParseError, r#trait::readable::FixedSizeReadable, Reader}};
use zerocopy::FromBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Copy, Clone, Default, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C, packed)]
pub struct Static {
  block: u16,
  size: u16,
  address_low: u32,
}

impl Static {
  pub fn index(&self) -> u8 { (self.block & 0x3F) as u8 }
  pub fn size(&self) -> u16 { self.size }
  pub fn address(&self) -> u64 {
    (((self.block & 0xFC0) as u64) << 30)
      | (((self.block & 0xF000) as u64) << 20)
      | (self.address_low as u64)
  }

  pub fn set_index(&mut self, index: u8) {
    if index > 0x3F {
      panic!("index too high! max is 0x3F, got: {}", index);
    }

    self.block &= 0xFFC0;
    self.block |= index as u16;
  }

  pub fn set_size(&mut self, size: u16) { self.size = size; }

  pub fn set_address(&mut self, address: u64) {
    if address > 0x3FFFFFFFFFF {
      panic!("address too high! max is 0x3FFFFFFFFFF, got: {}", address);
    }

    self.block &= 0x3F;
    self.block |= ((address >> 30) & 0x3F) as u16;
    self.block |= ((address >> 20) & 0xF) as u16;
    self.address_low = (address & 0xFFFFF) as u32;
  }

  pub fn with_index(mut self, index: u8) -> Self {
    self.set_index(index);
    self
  }
  pub fn with_size(mut self, size: u16) -> Self {
    self.set_size(size);
    self
  }
  pub fn with_address(mut self, address: u64) -> Self {
    self.set_address(address);
    self
  }

  pub fn new(index: u8, size: u16, address: u64) -> Self {
    Self::default()
      .with_index(index)
      .with_size(size)
      .with_address(address)
  }
}

pub enum StaticReadError {}

impl<const NODE_NAME_SIZE: usize> fileforge_lib::error::Error<NODE_NAME_SIZE> for StaticReadError {
  fn into_display(self) -> fileforge_lib::error::DisplayableError<NODE_NAME_SIZE, Self> where Self: Sized {
    unimplemented!()
  }

  fn with_report<Cb: FnMut(fileforge_lib::error::report::Report<NODE_NAME_SIZE>) -> ()>(&self, _: Cb) {
    unimplemented!()
  }
}

impl<'pool, const DIAGNOSTIC_NODE_NAME_SIZE: usize> FixedSizeReadable<'pool, DIAGNOSTIC_NODE_NAME_SIZE, 0x8> for Static {
  type Argument = ();
  type Error = StaticReadError;

  fn read<RP: Provider>(reader: &mut Reader<'pool, DIAGNOSTIC_NODE_NAME_SIZE, RP>, _: Self::Argument) -> Result<Self, ParseError<'pool, Self::Error, RP::ReadError, DIAGNOSTIC_NODE_NAME_SIZE>> {
    Ok(reader.with_dyn_bytes(Some(0x8), "Contents", |bytes| {
      <Self as FromBytes>::read_from(bytes).unwrap()
    })?)
  }
}