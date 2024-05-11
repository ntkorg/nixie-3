use crate::util::magic::MagicU32;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Copy, Clone, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct InHeader {
  magic: MagicU32,
  version: u32,
  command: u32,
  token: u32,
}
impl Default for InHeader {
  fn default() -> Self {
    Self {
      magic: b"SFCI".into(),
      version: 0,
      command: 0,
      token: 0,
    }
  }
}

impl InHeader {
  pub fn magic(&self) -> MagicU32 { self.magic }

  pub fn version(&self) -> u32 { self.version }

  pub fn command(&self) -> u32 { self.command }

  pub fn token(&self) -> u32 { self.token }

  pub fn set_magic(&mut self, magic: MagicU32) { self.magic = magic; }

  pub fn set_version(&mut self, version: u32) { self.version = version; }

  pub fn set_command(&mut self, command: u32) { self.command = command; }

  pub fn set_token(&mut self, token: u32) { self.token = token; }

  pub fn with_magic(mut self, magic: MagicU32) -> Self {
    self.set_magic(magic);
    self
  }

  pub fn with_version(mut self, version: u32) -> Self {
    self.set_version(version);
    self
  }

  pub fn with_command(mut self, command: u32) -> Self {
    self.set_command(command);
    self
  }

  pub fn with_token(mut self, token: u32) -> Self {
    self.set_token(token);
    self
  }

  pub fn new(magic: MagicU32, version: u32, command: u32, token: u32) -> Self {
    Self::default()
      .with_magic(magic)
      .with_version(version)
      .with_command(command)
      .with_token(token)
  }
}
