use zerocopy::{AsBytes, ByteSliceMut, FromBytes, FromZeroes};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use super::align::align_up;

pub enum ReadError {
    OutOfBounds,
    VecTooSmall,
    RequestedTooManyElements,
}

pub struct Reader<'a> {
    slice: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    pub fn new(slice: &'a [u8]) -> Self { Self { slice, offset: 0 } }

    pub fn read<T: Copy + FromBytes>(&mut self) -> Result<T, ReadError> {
        let size = core::mem::size_of::<T>();
        let slice = self.slice.get(self.offset..self.offset + size)
            .ok_or(ReadError::OutOfBounds)?;
        let value = T::read_from(slice).unwrap();
        self.offset += size;
        Ok(value)
    }

    pub fn read_vec<T: Copy + FromBytes, const N: usize>(
        &mut self,
        len: usize,
    ) -> Result<heapless::Vec<T, N>, ReadError> {
        let mut vec = heapless::Vec::new();
        self.read_into_vec(&mut vec, len)?;
        Ok(vec)
    }

    pub fn read_into_vec<T: Copy + FromBytes, const N: usize>(&mut self, vec: &mut heapless::Vec<T, N>, len: usize) -> Result<(), ReadError> {
        if len > N {
            return Err(ReadError::RequestedTooManyElements)
        }
        for _ in 0..len {
            vec.push(self.read()?)
                .map_err(|_| ReadError::VecTooSmall)?;
        }
        Ok(())
    }

    pub fn read_slice<T: Copy + FromBytes>(&mut self, len: usize) -> Result<&'a [T], ReadError> {
        let size = core::mem::size_of::<T>();
        let slice = self.slice.get(self.offset..self.offset + size * len)
            .ok_or(ReadError::OutOfBounds)?;
        self.offset += size * len;
        Ok(unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const T, len) })
    }

    pub fn align_to(&mut self, alignment: usize) { self.offset = align_up(self.offset, alignment); }
}
