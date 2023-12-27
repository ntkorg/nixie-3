use zerocopy_derive::{FromBytes, FromZeroes, AsBytes};

#[derive(Copy, Clone, Default, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct ReceiveStatic {
    address_low: u32,
    address_high: u16,
    size: u16,
}

impl ReceiveStatic {
    pub fn address(&self) -> u64 {
        ((self.address_high as u64) << 32) | (self.address_low as u64)
    }

    pub fn size(&self) -> u16 {
        self.size
    }

    pub fn set_address(&mut self, address: u64) {
        self.address_low = address as u32;
        self.address_high = (address >> 32) as u16;
    }

    pub fn set_size(&mut self, size: u16) {
        self.size = size;
    }

    pub fn with_address(mut self, address: u64) -> Self {
        self.set_address(address);
        self
    }

    pub fn with_size(mut self, size: u16) -> Self {
        self.set_size(size);
        self
    }

    pub fn new(address: u64, size: u16) -> Self {
        Self::default()
            .with_address(address)
            .with_size(size)
    }
}