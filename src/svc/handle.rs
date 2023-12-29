use core::{arch::asm, any::Any};

use crate::util::result::ResultCode;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Handle<T> {
    pub(crate) value: u32,
    _phantom: core::marker::PhantomData<T>,
}

impl<T> Handle<T> {
    pub(crate) fn new(value: u32) -> Self {
        Self {
            value,
            _phantom: Default::default(),
        }
    }

    pub fn close(self) {
        unsafe {
            asm!(
                "svc #0x16",
                in("w0") self.value,
            );
        }
    }
}

fn wait_synchronization<'a, T: Any + Sized>(
    handle: &'a [Handle<T>],
    timeout: core::time::Duration,
) -> Result<(), ResultCode> {
    let mut result;
    if timeout.as_nanos() > 0xFFFFFFFF {
        panic!("Timeout too large!");
    }
    unsafe {
        asm!(
            "svc #0x18",
            in("w0") handle.as_ptr() as u64,
            in("x1") timeout.as_nanos() as u64,
            lateout("w0") result,
        );
    }
    ResultCode::as_result(result)
}

pub trait Waitable: Sized {
    fn wait(value: Handle<Self>) -> Result<(), ResultCode> where Self: 'static {
        wait_synchronization(&[value], core::time::Duration::from_nanos(0))
    }
}
