use super::{
  CreateProcessParameter, CreateProcessParameterFlagsRaw, CreateProcessParameterRaw, Handle,
  Process,
};
use crate::result::result_code::ResultCode;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub unsafe fn create_process(
  parameters: &CreateProcessParameter,
  capability: &[u32],
) -> Result<Handle<Process>, ResultCode> {
  let mut error_code: usize;
  let mut handle_bits: u32;

  let flags_raw = CreateProcessParameterFlagsRaw::new()
    .with_is_64bit_instruction(parameters.is_64bit_instruction)
    .with_address_space_type(parameters.address_space_type as u8)
    .with_enable_debug(parameters.enable_debug)
    .with_enable_aslr(parameters.enable_aslr)
    .with_is_application(parameters.is_application)
    .with_use_secure_memory(parameters.use_secure_memory)
    .with_memory_region(parameters.memory_region as u8)
    .with_optimize_memory_allocation(parameters.optimize_memory_allocation);

  let raw = CreateProcessParameterRaw {
    name: parameters.name,
    category: (parameters.category as u32).into(),
    title_id: parameters.title_id.into(),
    code_addr: parameters.code_addr.into(),
    code_num_pages: parameters.code_num_pages.into(),
    flags: flags_raw.0.into(),
    resource_limit_handle: parameters.resource_limit_handle.into(),
    system_resource_num_pages: parameters.system_resource_num_pages.into(),
  };

  unsafe {
    asm!(
      "svc #0x79",

      in("x1") &raw as *const CreateProcessParameterRaw,
      in("x2") capability.as_ptr(),
      in("x3") capability.len(),
      lateout("x0") error_code,
      lateout("x1") handle_bits,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(Handle::from_bits(handle_bits));
  }

  Err(crate::result::result_code::ResultCode::from_bits(
    error_code as u32,
  ))
}
