use core::marker::PhantomData;

use zerocopy::FromZeroes;

use crate::ipc::sf::{self, service::Service};

use super::{dispdrv::IHOSBinderDriver, vi::{DisplayService, Layer}};

pub mod parcel;
pub mod graphics_buffer_producer;

pub struct Binder<B> {
  driver: Service<IHOSBinderDriver>,
  id: u32,
  _phantom: PhantomData<B>
}

impl<B> Binder<B> {


}
