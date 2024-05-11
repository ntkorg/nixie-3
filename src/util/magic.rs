use core::ptr::read_unaligned;

use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Copy, Clone, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C, packed)]
pub struct Magic<const WIDTH: usize> {
  magic: [u8; WIDTH],
}

impl<const WIDTH: usize> Magic<WIDTH> {
  pub fn magic(&self) -> [u8; WIDTH] { self.magic }

  pub fn new(magic: [u8; WIDTH]) -> Self { Self { magic: magic } }
}

impl<const IN_WIDTH: usize, const OUT_WIDTH: usize> From<&'static [u8; IN_WIDTH]>
  for Magic<OUT_WIDTH>
where
  [u8; OUT_WIDTH - IN_WIDTH]: Sized,
{
  fn from(value: &'static [u8; IN_WIDTH]) -> Self {
    let mut magic = [0; OUT_WIDTH];
    magic[..IN_WIDTH].copy_from_slice(value);
    Self::new(magic)
  }
}

pub fn reverse_magic<const IN_WIDTH: usize, const OUT_WIDTH: usize>(
  magic: &'static [u8; IN_WIDTH],
) -> Magic<OUT_WIDTH>
where
  [u8; OUT_WIDTH - IN_WIDTH]: Sized,
{
  let mut reversed = [0; OUT_WIDTH];
  reversed[..IN_WIDTH].copy_from_slice(magic);
  reversed.reverse();
  Magic::new(reversed)
}

pub type MagicU32 = Magic<4>;
