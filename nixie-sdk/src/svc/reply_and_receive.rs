use core::arch::asm;
use super::{Handle, ServerSession, ReplyAndReceiveError};
use crate::result::modules::svc;

#[cfg(target_pointer_width = "64")]
pub unsafe fn reply_and_receive<'a>(ports: &'a [Handle<ServerSession>], reply_target_session_handle: &Handle<ServerSession>, timeout: u64) -> Result<&'a Handle<ServerSession>, ReplyAndReceiveError<'a>> {
  let mut error_code: u32;
  let mut handle_index: u32;

  unsafe {
    asm!(
      "svc #0x43",
      
      in("x1") ports.as_ptr(),
      in("w2") ports.len(),
      in("w3") reply_target_session_handle.as_bits(),
      in("x4") timeout,
      lateout("w0") error_code,
      lateout("w1") handle_index,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(ports.get(handle_index as usize).unwrap());
  }

  let result_code = crate::result::result_code::ResultCode::from_bits(error_code as u32);

  if result_code == svc::SESSION_CLOSED {
    return Err(ReplyAndReceiveError::PortRemoteDead(ports.get(handle_index as usize).unwrap()))
  }

  Err(ReplyAndReceiveError::GenericResultCode(result_code))
}