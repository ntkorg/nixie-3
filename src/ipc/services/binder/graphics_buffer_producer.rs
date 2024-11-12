use core::marker::PhantomData;

use crate::ipc::{services::vi::{DisplayService, Layer}, sf::{self, service::Service}};

use super::Binder;

pub struct GraphicsBufferProducer;

impl Binder<GraphicsBufferProducer> {
  pub fn from_layer(
    layer: &Layer,
    display_service: &Service<DisplayService>,
  ) -> Result<Self, sf::Error> {
    Ok(Self {
      id: layer.binder_id,
      driver: display_service.get_relay_service()?,
      _phantom: PhantomData,
    })
  }
}
