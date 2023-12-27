use core::mem::MaybeUninit;

use zerocopy::{ByteSliceMut, AsBytes, FromBytes, FromZeroes};
use zerocopy::macro_util::{transmute_ref, transmute_mut};
use zerocopy_derive::{FromBytes, FromZeroes, AsBytes};

use super::align::align_up;

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
        value.write_to(&mut self.slice[self.offset..self.offset + size]);
        self.offset += size;
    }

    pub fn write_vec<T: Copy + AsBytes>(&mut self, value: &[T]) {
        let size = core::mem::size_of::<T>();
        value.write_to(&mut self.slice[self.offset..self.offset + size * value.len()]);
        self.offset += size * value.len();
    }

    pub fn align_to(&mut self, alignment: usize) {
        self.offset = align_up(self.offset, alignment);
    }
}
