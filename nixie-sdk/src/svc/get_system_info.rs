use core::arch::asm;
use crate::result::result_code::ResultCode;

#[cfg(target_pointer_width = "64")]
fn get_system_info_raw(handle_bits: u32, info_type: u32, info_subtype: u64) -> Result<u64, ResultCode> {
  let mut error_code: usize;
  let mut result: u64;

  unsafe {
    asm!(
      "svc #0x6F",
      
      in("w1") info_type,
      in("w2") handle_bits,
      in("x3") info_subtype,
      lateout("x0") error_code,
      lateout("x1") result,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(result);
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}

pub enum MemoryPool {
  Application = 0,
  Applet = 1,
  System = 2,
  SystemUnsafe = 3,
}

pub fn get_total_physical_memory_size(pool: MemoryPool) -> Result<u64, ResultCode> { get_system_info_raw(0, 0, pool as u64) }
pub fn get_used_physical_memory_size(pool: MemoryPool) -> Result<u64, ResultCode>  { get_system_info_raw(0, 1, pool as u64) }
pub fn get_initial_process_id_range_lower_bound() -> Result<u64, ResultCode>       { get_system_info_raw(0, 2, 0) }
pub fn get_initial_process_id_range_upper_bound() -> Result<u64, ResultCode>       { get_system_info_raw(0, 2, 1) }