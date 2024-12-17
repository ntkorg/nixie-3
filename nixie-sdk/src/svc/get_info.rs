use core::arch::asm;

use crate::result::result_code::ResultCode;

use super::{Handle, Process, Thread};

#[cfg(target_pointer_width = "64")]
fn get_info_raw(handle_bits: u32, info_type: u32, info_subtype: u64) -> Result<u64, ResultCode> {
  let mut error_code: usize;
  let mut result: u64;

  unsafe {
    asm!(
      "svc #0x29",

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

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}

pub fn get_core_mask(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 0, 0)
}
pub fn get_priority_mask(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 1, 0)
}
pub fn get_alias_region_address(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 2, 0)
}
pub fn get_alias_region_size(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 3, 0)
}
pub fn get_heap_region_address(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 4, 0)
}
pub fn get_heap_region_size(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 5, 0)
}
pub fn get_total_memory_size(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 6, 0)
}
pub fn get_used_memory_size(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 7, 0)
}
pub fn get_aslr_region_address(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 12, 0)
}
pub fn get_aslr_region_size(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 13, 0)
}
pub fn get_stack_region_address(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 14, 0)
}
pub fn get_stack_region_size(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 15, 0)
}
pub fn get_system_resource_size_total(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 16, 0)
}
pub fn get_system_resource_size_used(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 17, 0)
}
pub fn get_program_id(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 18, 0)
}
pub fn get_initial_process_id_range_lower_bound(
  process: Handle<Process>,
) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 19, 0)
}
pub fn get_initial_process_id_range_upper_bound(
  process: Handle<Process>,
) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 19, 1)
}
pub fn get_user_exception_context_address(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 20, 0)
}
pub fn get_total_non_system_memory_size(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 21, 0)
}
pub fn get_used_non_system_memory_size(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 22, 0)
}
pub fn is_application(process: Handle<Process>) -> Result<bool, ResultCode> {
  get_info_raw(process.as_bits(), 23, 0).map(|e| e != 0)
}
pub fn get_free_thread_count(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 24, 0)
}
pub fn is_svc_permitted(process: Handle<Process>) -> Result<bool, ResultCode> {
  get_info_raw(process.as_bits(), 26, 0).map(|e| e != 0)
}
pub fn get_io_region_hint(process: Handle<Process>) -> Result<u64, ResultCode> {
  get_info_raw(process.as_bits(), 27, 0)
}

pub fn get_thread_tick_count(
  thread: Handle<Thread>,
  core_idx: Option<i64>,
) -> Result<u64, ResultCode> {
  get_info_raw(thread.as_bits(), 25, core_idx.unwrap_or(-1) as u64)
}

pub fn is_debugger_attached() -> Result<bool, ResultCode> { get_info_raw(0, 8, 0).map(|v| v != 0) }
pub fn get_resource_limit() -> Result<u64, ResultCode> { get_info_raw(0, 9, 0) }
pub fn get_idle_tick_count() -> Result<u64, ResultCode> { get_info_raw(0, 10, -1 as i64 as u64) }
pub fn get_random_entropy() -> Result<[u64; 4], ResultCode> {
  Ok([
    get_info_raw(0, 11, 0)?,
    get_info_raw(0, 11, 1)?,
    get_info_raw(0, 11, 2)?,
    get_info_raw(0, 11, 3)?,
  ])
}
