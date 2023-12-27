use zerocopy_derive::{FromBytes, FromZeroes, AsBytes};

#[derive(Copy, Clone, Default, Debug)]
pub enum Mode {
    #[default]
    Normal = 0,
    NonSecure = 1,
    Invalid = 2,
    NonDevice = 3,
}

#[derive(Copy, Clone, Default, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C, packed)]
pub struct Buffer {
    size_low: u32,
    address_low: u32,
    block: u32,
}

impl Buffer {
    pub fn size(&self) -> u64 {
        ((self.block & 0xFFFF_FFFF) as u64) | (((self.block >> 28) & 0xF) as u64) << 32
    }

    pub fn address(&self) -> u64 {
        (((self.block >> 2) & 0x3FFFFF) as u64) << 12
            | (((self.block >> 26) & 0xF) as u64) << 44
            | (self.address_low as u64)
    }

    pub fn mode(&self) -> Mode {
        match (self.block >> 24) & 0b11 {
            0 => Mode::Normal,
            1 => Mode::NonSecure,
            2 => Mode::Invalid,
            3 => Mode::NonDevice,
            _ => unreachable!(),
        }
    }

    pub fn set_size(&mut self, size: u64) {
        if size > 0xFFFF_FFFF_FFFF {
            panic!("size too high! max is 0xFFFF_FFFF_FFFF, got: {}", size);
        }

        self.block &= 0xF000_0000;
        self.block |= (size & 0xFFFF_FFFF) as u32;
        self.block |= ((size >> 32) & 0xF) as u32;
    }

    pub fn set_address(&mut self, address: u64) {
        if address > 0x3FFFFFFFFFF {
            panic!("address too high! max is 0x3FFFFFFFFFF, got: {}", address);
        }

        self.block &= 0x3F;
        self.block |= ((address >> 12) & 0x3FFFFF) as u32;
        self.block |= ((address >> 44) & 0xF) as u32;
        self.address_low = (address & 0xFFF) as u32;
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.block &= 0xFCFFFFFF;
        self.block |= match mode {
            Mode::Normal => 0,
            Mode::NonSecure => 1,
            Mode::Invalid => 2,
            Mode::NonDevice => 3,
        } << 24;
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.set_size(size);
        self
    }

    pub fn with_address(mut self, address: u64) -> Self {
        self.set_address(address);
        self
    }

    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.set_mode(mode);
        self
    }

    pub fn new(size: u64, address: u64, mode: Mode) -> Self {
        Self::default()
            .with_size(size)
            .with_address(address)
            .with_mode(mode)
    }
}
