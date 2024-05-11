use core::arch::asm;

use crate::util::result::ResultCode;

use super::Handle;

#[derive(Copy, Clone)]
pub struct Session();

impl Handle<Session> {
  pub fn send_sync_request(&self) -> Result<(), ResultCode> {
    let mut result;
    unsafe {
      asm!(
          "svc #0x21",
          in("w0") self.value,
          lateout("w0") result,
      );
    }
    ResultCode::as_result(result)
  }

  pub fn connect_to_named_port<const LEN: usize>(
    name: &'static [u8; LEN],
  ) -> Result<Handle<Session>, ResultCode> {
    let mut handle;
    let mut result;
    if LEN >= 12 {
      panic!("Port name too long! Must be less than 12 bytes.")
    }

    let mut c_name = [0u8; 12];
    c_name[..LEN].copy_from_slice(name);

    unsafe {
      asm!(
          "svc #0x1F",
          in("x1") c_name.as_ptr(),
          lateout("w0") result,
          lateout("w1") handle,
      );
    }

    ResultCode::as_result(result).map(|_| Handle::new(handle))
  }
}
