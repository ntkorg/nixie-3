use super::{Handle, ReplyAndReceiveError, ServerSession};
use crate::result::modules::svc;
use core::arch::asm;

#[cfg(target_pointer_width = "64")]
pub unsafe fn reply_and_receive_with_user_buffer<'a>(
  user_buffer: &mut [u8],
  ports: &'a [Handle<ServerSession>],
  reply_target_session_handle: &Handle<ServerSession>,
  timeout: u64,
) -> Result<&'a Handle<ServerSession>, ReplyAndReceiveError<'a>> {
  let mut error_code: u32;
  let mut handle_index: u32;

  unsafe {
    asm!(
      "svc #0x44",

      in("x1") user_buffer.as_mut_ptr(),
      in("x2") user_buffer.len(),
      in("x3") ports.as_ptr(),
      in("w4") ports.len(),
      in("w5") reply_target_session_handle.as_bits(),
      in("x6") timeout,
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
    return Err(ReplyAndReceiveError::PortRemoteDead(
      ports.get(handle_index as usize).unwrap(),
    ));
  }

  Err(ReplyAndReceiveError::GenericResultCode(result_code))
}
