use zerocopy_derive::{FromBytes, FromZeroes, AsBytes};

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
            panic!(
                "index too high! max is 0x3F, got: {}",
                index
            );
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
