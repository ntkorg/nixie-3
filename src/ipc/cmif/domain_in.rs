use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Copy, Clone, Debug)]
pub enum DomainCommandType {
  Request = 0,
  Close = 1,
}

#[derive(Copy, Clone, Default, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct DomainInHeader {
  pub command_type: u8,
  pub object_count: u8,
  pub data_size: u16,
  pub object_id: u32,
  pub padding: u32,
  pub token: u32,
}

impl DomainInHeader {
  pub fn command_type(&self) -> DomainCommandType {
    match self.command_type {
      0 => DomainCommandType::Request,
      1 => DomainCommandType::Close,
      _ => panic!("Invalid command type"),
    }
  }

  pub fn object_count(&self) -> u8 { self.object_count }

  pub fn data_size(&self) -> u16 { self.data_size }

  pub fn object_id(&self) -> u32 { self.object_id }

  pub fn token(&self) -> u32 { self.token }

  pub fn set_command_type(&mut self, command_type: DomainCommandType) {
    self.command_type = match command_type {
      DomainCommandType::Request => 0,
      DomainCommandType::Close => 1,
    };
  }

  pub fn set_object_count(&mut self, object_count: u8) { self.object_count = object_count; }

  pub fn set_data_size(&mut self, data_size: u16) { self.data_size = data_size; }

  pub fn set_object_id(&mut self, object_id: u32) { self.object_id = object_id; }

  pub fn set_token(&mut self, token: u32) { self.token = token; }

  pub fn with_command_type(mut self, command_type: DomainCommandType) -> Self {
    self.set_command_type(command_type);
    self
  }

  pub fn with_object_count(mut self, object_count: u8) -> Self {
    self.set_object_count(object_count);
    self
  }

  pub fn with_data_size(mut self, data_size: u16) -> Self {
    self.set_data_size(data_size);
    self
  }

  pub fn with_object_id(mut self, object_id: u32) -> Self {
    self.set_object_id(object_id);
    self
  }

  pub fn with_token(mut self, token: u32) -> Self {
    self.set_token(token);
    self
  }

  pub fn new(
    command_type: DomainCommandType,
    object_count: u8,
    data_size: u16,
    object_id: u32,
    token: u32,
  ) -> Self {
    Self::default()
      .with_command_type(command_type)
      .with_object_count(object_count)
      .with_data_size(data_size)
      .with_object_id(object_id)
      .with_token(token)
  }
}
