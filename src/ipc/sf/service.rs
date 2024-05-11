use core::ops::Deref;

use heapless::String;

use crate::{
  ipc::sf::{invoke_service_control, Error, RequestContext},
  svc::{output_debug_string, panic, Handle, Session},
  util::tls,
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Service<T> {
  pub(crate) handle: Handle<Session>,
  pub(crate) object: Option<u32>,
  pub(crate) is_root: bool,
  _phantom: core::marker::PhantomData<T>,
}

impl<T> Service<T> {
  pub fn new(handle: Handle<Session>) -> Self {
    Self {
      handle,
      object: None,
      is_root: true,
      _phantom: Default::default(),
    }
  }

  pub fn is_domain(&self) -> bool { self.object.is_some() }

  pub fn subservice<O>(&self, object: u32) -> Service<O> {
    Service::<O> {
      handle: self.handle,
      object: Some(object),
      is_root: false,
      _phantom: Default::default(),
    }
  }

  pub fn convert_to_domain(self) -> Result<Self, Error> {
    assert!(self.object.is_none());
    invoke_service_control(
      &self,
      &RequestContext::new().with_command(0).with_token(0),
      |_, res| {
        let data = res.get_data();

        output_debug_string("kiully soreyfl");
        tls::dump();

        // panic(1024, if data.len() < 4 { 1 } else {0 }, data.len());

        if data.len() < 4 {
          return Err(Error::NotEnoughData);
        }

        let object = u32::from_le_bytes(data[..4].try_into().unwrap());
        Ok(Service {
          handle: self.handle,
          object: Some(object),
          is_root: false,
          _phantom: Default::default(),
        })
      },
    )
  }

  pub fn copy_from_current_domain(&self) -> Result<Self, Error> {
    assert!(self.object.is_some());
    invoke_service_control(&self, &RequestContext::new().with_command(1), |_, res| {
      let data = res.get_data();
      if data.len() != 4 {
        return Err(Error::NotEnoughData);
      }

      let object = u32::from_le_bytes(data.try_into().unwrap());
      Ok(Service {
        handle: self.handle,
        object: Some(object),
        is_root: false,
        _phantom: Default::default(),
      })
    })
  }

  pub fn query_pointer_size(&self) -> Result<u32, ()> {
    // atmosphere never returns >0, and therefore we can assume 0
    // though none of the service code actually handles it lol
    Ok(0)
  }
}
