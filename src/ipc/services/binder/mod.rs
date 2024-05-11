use core::marker::PhantomData;

use zerocopy::FromZeroes;

use crate::ipc::sf::{self, service::Service};

use super::{dispdrv::IHOSBinderDriver, vi::{DisplayService, Layer}};

pub mod parcel;

pub struct Binder<B> {
  driver: Service<IHOSBinderDriver>,
  id: u32,
  _a: PhantomData<B>
}

impl<B> Binder<B> {
  pub fn from_layer(layer: &Layer, display_service: Service<DisplayService>) -> Result<Self, sf::Error> {
    Ok(Self {
      id: layer.binder_id,
      driver: display_service.get_relay_service()?,
      _a: PhantomData
    })
  }


}
