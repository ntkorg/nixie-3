use core::mem::MaybeUninit;

use zerocopy::macro_util::{transmute_mut, transmute_ref};
use zerocopy::{AsBytes, ByteSliceMut, FromBytes, FromZeroes};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use super::align::align_up;

#[derive(Debug)]
pub enum WriteError {
  TooMuchData,
  OutOfBounds,
}

pub struct Writer<'a> {
  slice: &'a mut [u8],
  offset: usize,
}

impl<'a> Writer<'a> {
  pub fn new(slice: &'a mut [u8]) -> Self { Self { slice, offset: 0 } }

  pub fn write<T: Copy + AsBytes>(&mut self, value: T) -> Result<(), WriteError> {
    let size = core::mem::size_of::<T>();

    let slice = self
      .slice
      .get_mut(self.offset..self.offset + size)
      .ok_or(WriteError::OutOfBounds)?;
    value.write_to(slice).ok_or(WriteError::TooMuchData)?;
    self.offset += size;
    Ok(())
  }

  pub fn write_vec<T: Copy + AsBytes>(&mut self, value: &[T]) -> Result<(), WriteError> {
    let size = core::mem::size_of::<T>();
    let slice = self
      .slice
      .get_mut(self.offset..self.offset + size * value.len())
      .ok_or(WriteError::OutOfBounds)?;
    value.write_to(slice).ok_or(WriteError::TooMuchData)?;
    self.offset += size * value.len();
    Ok(())
  }

  pub fn align_to(&mut self, alignment: usize) {
    let end = align_up(self.offset, alignment);
    // for safety and less debugging confusion, fill the padding with zeroes
    self
      .slice
      .get_mut(self.offset..end.max(self.slice.len()))
      .map(|slice| slice.fill(0));
    self.offset = end;
  }
}
