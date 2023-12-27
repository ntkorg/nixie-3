use zerocopy_derive::{FromBytes, FromZeroes, AsBytes};

#[derive(Copy, Clone, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C, packed)]
pub struct Magic<const WIDTH: usize> {
    magic: [u8; WIDTH]
}

impl<const WIDTH: usize> Magic<WIDTH> {
    pub fn magic(&self) -> [u8; WIDTH] {
        self.magic
    }

    pub fn new(magic: [u8; WIDTH]) -> Self {
        Self { magic }
    }
}

pub type MagicU32 = Magic<4>;
