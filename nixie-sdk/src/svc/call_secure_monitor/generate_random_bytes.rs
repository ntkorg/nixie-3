use super::{call_secure_monitor, SmcResult, SecureMonitorArgument::{ Value, Null }};

pub fn generate_random_bytes(buffer: &mut [u8]) -> Result<(), SmcResult> {
  if buffer.len() > 56 {
    return Err(SmcResult::InvalidArgument);
  }

  let u64_slice = call_secure_monitor(6, [Value(buffer.len() as u64), Null, Null, Null, Null, Null, Null])?;
  let u8_slice = unsafe { core::mem::transmute::<[u64; 7], [u8; 56]>(u64_slice) };

  buffer.copy_from_slice(&u8_slice[0..buffer.len()]);

  Ok(())
}