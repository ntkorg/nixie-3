use fileforge_lib::{provider::r#trait::Provider, reader::{error::parse::ParseError, r#trait::readable::FixedSizeReadable, Reader}};
use zerocopy::FromBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Copy, Clone, Default, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C, packed)]
pub struct SpecialHeader {
  pub block: u32,
}

impl SpecialHeader {
  pub fn has_pid(&self) -> bool { (self.block & 0b1) != 0 }
  pub fn copy_handle_count(&self) -> u8 { ((self.block >> 1) & 0xF) as u8 }
  pub fn move_handle_count(&self) -> u8 { ((self.block >> 5) & 0xF) as u8 }

  pub fn set_has_pid(&mut self, pid: bool) {
    self.block &= 0xFFFFFFFE;
    self.block |= if pid { 1 } else { 0 }
  }

  pub fn set_copy_handle_count(&mut self, copy_handle_count: u8) {
    if copy_handle_count > 0xF {
      panic!(
        "copy_handle_count too high! max is 0xF, got: {}",
        copy_handle_count
      );
    }
    self.block &= 0xFFFFFFE1;
    self.block |= (copy_handle_count as u32) << 1;
  }

  pub fn set_move_handle_count(&mut self, move_handle_count: u8) {
    if move_handle_count > 0xF {
      panic!(
        "move_handle_count too high! max is 0xF, got: {}",
        move_handle_count
      );
    }

    self.block &= 0xFFFFFE1F;
    self.block |= (move_handle_count as u32) << 5;
  }

  pub fn with_has_pid(mut self, has_pid: bool) -> Self {
    self.set_has_pid(has_pid);
    self
  }
  pub fn with_copy_handle_count(mut self, copy_handle_count: u8) -> Self {
    self.set_copy_handle_count(copy_handle_count);
    self
  }
  pub fn with_move_handle_count(mut self, move_handle_count: u8) -> Self {
    self.set_move_handle_count(move_handle_count);
    self
  }

  pub fn new(has_pid: bool, copy_handle_count: u8, move_handle_count: u8) -> Self {
    Self::default()
      .with_has_pid(has_pid)
      .with_copy_handle_count(copy_handle_count)
      .with_move_handle_count(move_handle_count)
  }
}

pub enum SpecialHeaderReadError {}

impl<const NODE_NAME_SIZE: usize> fileforge_lib::error::Error<NODE_NAME_SIZE> for SpecialHeaderReadError {
  fn into_display(self) -> fileforge_lib::error::DisplayableError<NODE_NAME_SIZE, Self> where Self: Sized {
    unimplemented!()
  }

  fn with_report<Cb: FnMut(fileforge_lib::error::report::Report<NODE_NAME_SIZE>) -> ()>(&self, _: Cb) {
    unimplemented!()
  }
}

impl<'pool, const DIAGNOSTIC_NODE_NAME_SIZE: usize> FixedSizeReadable<'pool, DIAGNOSTIC_NODE_NAME_SIZE, 4> for SpecialHeader {
  type Argument = ();
  type Error = SpecialHeaderReadError;

  fn read<RP: Provider>(reader: &mut Reader<'pool, DIAGNOSTIC_NODE_NAME_SIZE, RP>, _: Self::Argument) -> Result<Self, ParseError<'pool, Self::Error, RP::ReadError, DIAGNOSTIC_NODE_NAME_SIZE>> {
    Ok(reader.with_dyn_bytes(Some(4), "Contents", |bytes| {
      <Self as FromBytes>::read_from(bytes).unwrap()
    })?)
  }
}