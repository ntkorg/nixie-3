use core::fmt::{Debug, Display};

use bitfield_struct::bitfield;

use super::result_code_modules::{ResultCodeDescription, ResultCodeModule, RESULT_CODE_MODULES};

#[bitfield(u32, debug = false)]
pub struct ResultCode {
  #[bits(9)]
  pub module: u16,

  #[bits(13)]
  pub code: u16,

  #[bits(10)]
  unused: u16,
}

impl ResultCode {
  pub fn resolve_module(&self) -> Option<&ResultCodeModule> {
    RESULT_CODE_MODULES.get(self.module() as usize)?.as_ref()
  }

  pub fn resolve_code(&self) -> Option<ResultCodeDescription> {
    self.resolve_module()
      .map(|m| (m.get_result_description)(self.code() as u32))
      .flatten()
  }

  pub const fn of(module_id: u16, code_id: u16) -> ResultCode {
    ResultCode::new().with_module(module_id).with_code(code_id)
  }

  pub fn expect(&self, code: ResultCode) {
    if *self != code {
      panic!("expected {}, got {:?}", code, self);
    }
  }
}

impl Display for ResultCode {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if self.unused() == 0 {
      match (self.resolve_module(), self.resolve_code()) {
        (None, None) => f.write_fmt(format_args!("ResultCode ({:#x}) {{ module: Unknown({}), code: Unknown({}) }}", self.0, self.module(), self.code())),
        (Some(module), None) => f.write_fmt(format_args!("ResultCode ({:#x}) {{ module: {}, code: Unknown({}) }}", self.0, module.name, self.code())),
        (None, Some(code)) => f.write_fmt(format_args!("ResultCode ({:#x}) {{ module: Unknown({}), code: {} }}", self.0, self.module(), code.name)),
        (Some(module), Some(code)) => f.write_fmt(format_args!("ResultCode ({:#x}) {{ module: {}, code: {} }}", self.0, module.name, code.name)),
      }
    } else {
      match (self.resolve_module(), self.resolve_code()) {
        (None, None) => f.write_fmt(format_args!("ResultCode ({:#x}) {{ module: Unknown({}), code: Unknown({}), unknown: {} }}", self.0, self.module(), self.code(), self.unused())),
        (Some(module), None) => f.write_fmt(format_args!("ResultCode ({:#x}) {{ module: {}, code: Unknown({}), unknown: {} }}", self.0, module.name, self.code(), self.unused())),
        (None, Some(code)) => f.write_fmt(format_args!("ResultCode ({:#x}) {{ module: Unknown({}), code: {}, unknown: {} }}", self.0, self.module(), code.name, self.unused())),
        (Some(module), Some(code)) => f.write_fmt(format_args!("ResultCode ({:#x}) {{ module: {}, code: {}, unknown: {} }}", self.0, module.name, code.name, self.unused())),
      }
    }
  }
}

impl Debug for ResultCode {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if self.unused() == 0 {
      match (self.resolve_module(), self.resolve_code()) {
        (None, None) => f.write_fmt(format_args!("ResultCode ({:#x}) {{\n  module: Unknown({}),\n  code: Unknown({})\n}}", self.0, self.module(), self.code())),
        (Some(module), None) => f.write_fmt(format_args!("ResultCode ({:#x}) {{\n  // {}\n  module: {},\n  code: Unknown({})\n}}", self.0, module.description, module.name, self.code())),
        (None, Some(code)) => f.write_fmt(format_args!("ResultCode ({:#x}) {{\n  module: Unknown({}),\n\n  // {}\n  code: {}\n}}", self.0, self.module(), code.description, code)),
        (Some(module), Some(code)) => f.write_fmt(format_args!("ResultCode ({:#x}) {{\n  // {}\n  module: {},\n\n  // {}\n  code: {}\n}}", self.0, module.description, module.name, code.description, code)),
      }
    } else {
      match (self.resolve_module(), self.resolve_code()) {
        (None, None) => f.write_fmt(format_args!("ResultCode ({:#x}) {{\n  module: Unknown({}),\n  code: Unknown({}),\n  unknown: {}\n}}", self.0, self.module(), self.code(), self.unused())),
        (Some(module), None) => f.write_fmt(format_args!("ResultCode ({:#x}) {{\n  // {}\n  module: {},\n  code: Unknown({}),\n  unknown: {}\n}}", self.0, module.description, module.name, self.code(), self.unused())),
        (None, Some(code)) => f.write_fmt(format_args!("ResultCode ({:#x}) {{\n  module: Unknown({}),\n\n  // {}\n  code: {}\n\n  unknown: {}\n}}", self.0, self.module(), code.description, code, self.unused())),
        (Some(module), Some(code)) => f.write_fmt(format_args!("ResultCode ({:#x}) {{\n  // {}\n  module: {},\n\n  // {}\n  code: {}\n\n  unknown: {}\n}}", self.0, module.description, module.name, code.description, code, self.unused())),
      }
    }
  }
}

impl Eq for ResultCode {}
impl PartialEq for ResultCode {
  fn eq(&self, other: &Self) -> bool {
    self.module() == other.module() && self.code() == other.code()
  }
}