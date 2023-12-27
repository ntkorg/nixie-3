use zerocopy::{AsBytes, ByteSliceMut, FromBytes, FromZeroes};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use super::align::align_up;

pub struct Reader<'a> {
    slice: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    pub fn new(slice: &'a [u8]) -> Self { Self { slice, offset: 0 } }

    pub fn read<T: Copy + FromBytes>(&mut self) -> T {
        let size = core::mem::size_of::<T>();
        let value = T::read_from(&self.slice[self.offset..self.offset + size])
            .expect("Failed to read value");
        self.offset += size;
        value
    }

    pub fn read_vec<T: Copy + FromBytes, const N: usize>(
        &mut self,
        len: usize,
    ) -> heapless::Vec<T, N> {
        let mut vec = heapless::Vec::new();
        self.read_into_vec(&mut vec, len);
        vec
    }

    pub fn read_into_vec<T: Copy + FromBytes, const N: usize>(&mut self, vec: &mut heapless::Vec<T, N>, len: usize) {
        if len > N {
            panic!("Requested length is larger than the maximum length");
        }
        for _ in 0..len {
            vec.push(self.read())
                .map_err(|_| ())
                .expect("Failed to read value");
        }
    }

    pub fn read_slice<T: Copy + FromBytes>(&mut self, len: usize) -> &'a [T] {
        let size = core::mem::size_of::<T>();
        let slice = &self.slice[self.offset..self.offset + size * len];
        self.offset += size * len;
        unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const T, len) }
    }

    pub fn align_to(&mut self, alignment: usize) { self.offset = align_up(self.offset, alignment); }
}
