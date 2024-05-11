use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

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
    // ((self.block & 0xFFFF_FFFF) as u64) | (((self.block >> 28) & 0xF) as u64) << 32
    todo!("rewrite this");
  }

  pub fn address(&self) -> u64 {
    todo!("rewrite this");
    // (((self.block >> 2) & 0x3FFFFF) as u64) << 12
    //   | (((self.block >> 26) & 0xF) as u64) << 44
    //   | (self.address_low as u64)
  }

  pub fn mode(&self) -> Mode {
    match (self.block) & 0b11 {
      0 => Mode::Normal,
      1 => Mode::NonSecure,
      2 => Mode::Invalid,
      3 => Mode::NonDevice,
      _ => unreachable!(),
    }
  }

  pub fn set_size(&mut self, size: u64) {
    if size > 0xF_FFFF_FFFF {
      panic!("size too high! max is 0xF_FFFF_FFFF, got: {}", size);
    }

    self.size_low = (size & 0xFFFF_FFFF) as u32;
    self.block &= 0x0F00_0000;
    self.block |= ((size >> 8) & 0x0F00_0000) as u32;
  }

  pub fn set_address(&mut self, address: u64) {
    if address > 0x7F_FFFF_FFFF {
      panic!("address too high! max is 0x7F_FFFF_FFFF, got: {}", address);
    }

    self.address_low = (address & 0xFFFF_FFFF) as u32;
    self.block &= 0xF000_001C;
    self.block |= ((address >> 4) & 0xF000_0000) as u32;
    self.block |= ((address >> 34) & 0x0000_001C) as u32;
  }

  pub fn set_mode(&mut self, mode: Mode) {
    self.block &= 0x0000_0003;
    self.block |= match mode {
      Mode::Normal => 0,
      Mode::NonSecure => 1,
      Mode::Invalid => 2,
      Mode::NonDevice => 3,
    };
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
