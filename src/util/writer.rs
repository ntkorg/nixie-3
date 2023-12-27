use zerocopy::transmute_mut;
use zerocopy_derive::{FromBytes, FromZeroes, AsBytes};

pub struct Writer<'a> {
    slice: &'a mut [u8],
    offset: usize,
}

impl<'a> Writer<'a> {
    pub fn new(slice: &'a mut [u8]) -> Self {
        Self {
            slice,
            offset: 0,
        }
    }

    pub fn write<T: Copy + AsBytes>(&mut self, value: T) {
        let size = core::mem::size_of::<T>();
        let slice = &mut self.slice[self.offset..self.offset + size];
        self.offset += size;
        transmute_mut!(slice)[0] = value;
    }
}
