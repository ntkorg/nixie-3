use core::arch::asm;

use crate::util::result::ResultCode;

use super::handle::Handle;

#[derive(Copy, Clone)]
pub struct Thread;

impl Handle<Thread> {
    pub fn start(&self) -> Result<(), ResultCode> {
        let mut result;
        unsafe {
            asm!(
                "svc #0x0",
                in("w0") self.value,
                lateout("w0") result,
            );
        }
        ResultCode::as_result(result)
    }

    pub fn wait_for_exit(&self) -> Result<(), ResultCode> {
        let mut result;
        unsafe {
            asm!(
                "svc #0x09",
                in("w0") self.value,
                lateout("w0") result,
            );
        }
        ResultCode::as_result(result)
    }

    pub fn get_handle(&self) -> Handle<Thread> { Handle::new(self.value) }

    pub fn create(
        entrypoint: extern "C" fn(usize) -> !,
        arg: usize,
        stack_top: usize,
        priority: i32,
        processor_id: i32,
    ) -> Result<Handle<Thread>, ResultCode> {
        let mut handle;
        let mut result;
        unsafe {
            asm!(
                "svc #0x08",
                in("x0") entrypoint,
                in("x1") arg,
                in("x2") stack_top,
                in("w3") priority,
                in("w4") processor_id,
                lateout("w0") result,
                lateout("w1") handle,
            );
        }
        ResultCode::as_result(result).map(|_| Handle::new(handle))
    }
}
