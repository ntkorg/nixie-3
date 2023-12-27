use core::ops::Deref;

use crate::svc::{Handle, Session};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Service<T> {
    pub(crate) handle: Handle<Session>,
    pub(crate) object: Option<u32>,
    pub(crate) is_root: bool,
    _phantom: core::marker::PhantomData<T>,
}

impl<T> Service<T> {
    fn new(handle: Handle<Session>) -> Self {
        Self {
            handle,
            object: None,
            is_root: true,
            _phantom: Default::default(),
        }
    }   

    fn domain_subservice(&self, object: u32) -> Self {
        Self {
            handle: self.handle,
            object: Some(object),
            is_root: true,
            _phantom: Default::default(),
        }
    }
}
