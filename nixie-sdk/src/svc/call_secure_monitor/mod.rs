// TODO: SetConfig (undocumented?)
pub mod generate_random_bytes;
pub mod get_config;

// Core

use bitfield_struct::bitfield;
use core::{arch::asm, ffi::c_void, fmt::Debug};

enum SecureMonitorArgument {
  Value(u64),
  Pointer(*mut c_void),
  Null,
}

impl SecureMonitorArgument {
  fn is_pointer(&self) -> bool {
    if let Self::Pointer(..) = self {
      true
    } else {
      false
    }
  }

  fn get_value_as_u64(&self) -> u64 {
    match self {
      Self::Value(v) => *v,
      Self::Pointer(v) => *v as u64,
      Self::Null => 0,
    }
  }
}

#[bitfield(u32)]
struct SmcFunctionId {
  function_number: u8,
  register_x0_is_pointer: bool,
  register_x1_is_pointer: bool,
  register_x2_is_pointer: bool,
  register_x3_is_pointer: bool,
  register_x4_is_pointer: bool,
  register_x5_is_pointer: bool,
  register_x6_is_pointer: bool,
  register_x7_is_pointer: bool,
  reserved: u8,

  #[bits(6)]
  call_range: u8,

  call_convention_is_64bit: bool,
  is_fast_call: bool,
}

#[derive(Debug)]
pub enum SmcResult {
  NotImplemented,
  InvalidArgument,
  InProgress,
  NoAsyncOperation,
  InvalidAsyncOperation,
  NotPermitted,
}

#[cfg(target_pointer_width = "64")]
fn call_secure_monitor(
  function_number: u8,
  arguments: [SecureMonitorArgument; 7],
) -> Result<[u64; 7], SmcResult> {
  let mut error_code: u32;
  let mut output_0: u64 = 0;
  let mut output_1: u64 = 0;
  let mut output_2: u64 = 0;
  let mut output_3: u64 = 0;
  let mut output_4: u64 = 0;
  let mut output_5: u64 = 0;
  let mut output_6: u64 = 0;

  let function_id = SmcFunctionId::new()
    .with_call_convention_is_64bit(true)
    .with_call_range(3) // OEM Service
    .with_is_fast_call(true)
    .with_reserved(0)
    .with_function_number(function_number)
    .with_register_x0_is_pointer(false)
    .with_register_x1_is_pointer(arguments[0].is_pointer())
    .with_register_x2_is_pointer(arguments[1].is_pointer())
    .with_register_x3_is_pointer(arguments[2].is_pointer())
    .with_register_x4_is_pointer(arguments[3].is_pointer())
    .with_register_x5_is_pointer(arguments[4].is_pointer())
    .with_register_x6_is_pointer(arguments[5].is_pointer())
    .with_register_x7_is_pointer(arguments[6].is_pointer());

  unsafe {
    asm!(
      "svc #0x7F",

      in("x0") function_id.0,
      in("x1") arguments[0].get_value_as_u64(),
      in("x2") arguments[1].get_value_as_u64(),
      in("x3") arguments[2].get_value_as_u64(),
      in("x4") arguments[3].get_value_as_u64(),
      in("x5") arguments[4].get_value_as_u64(),
      in("x6") arguments[5].get_value_as_u64(),
      in("x7") arguments[6].get_value_as_u64(),
      lateout("w0") error_code,
      lateout("w1") output_0,
      lateout("w2") output_1,
      lateout("x3") output_2,
      lateout("x4") output_3,
      lateout("x5") output_4,
      lateout("x6") output_5,
      lateout("x7") output_6,
    );
  }

  match error_code {
    0 => Ok([
      output_0, output_1, output_2, output_3, output_4, output_5, output_6,
    ]),
    1 => Err(SmcResult::NotImplemented),
    2 => Err(SmcResult::InvalidArgument),
    3 => Err(SmcResult::InProgress),
    4 => Err(SmcResult::NoAsyncOperation),
    5 => Err(SmcResult::InvalidAsyncOperation),
    6 => Err(SmcResult::NotPermitted),

    _ => panic!(
      "Invalid SMC Result code: {} (x1: {}, x2: {}, x3: {}, x4: {}, x5: {}, x6: {}, x7: {})",
      error_code, output_0, output_1, output_2, output_3, output_4, output_5, output_6
    ),
  }
}
