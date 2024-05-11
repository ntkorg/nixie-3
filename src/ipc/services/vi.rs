use heapless::String;
use zerocopy::FromBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::{
  ipc::{
    services::binder::parcel::ParcelHeader, sf::{self, service::Service, Error, RequestContext}
  },
  util::result::ResultCode,
};

use super::{dispdrv::IHOSBinderDriver, sm::ServiceName, ServiceRoot};

pub struct ViUser;

impl ServiceRoot for ViUser {
  fn name() -> ServiceName { b"vi:u".into() }
  fn is_domain() -> bool { false }
}

impl Service<ViUser> {
  pub fn get_display_service(&self) -> Result<Service<DisplayService>, sf::Error> {
    let req = RequestContext::new_from(&0).with_command(0);

    sf::invoke_service_request(self, &req, |_, mut res| {
      Ok(Service::new(res.get_move_handle()?))
    })
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayId(pub(crate) u64, [u8; 0x40]);

#[repr(align(8))]
pub struct Layer {
  pub native_window: [u8; 0x100],
  pub layer_id: u64,
  pub binder_id: u32,
}

pub struct DisplayService;

impl Service<DisplayService> {
  pub fn open_display(&self, name: String<0x40>) -> Result<DisplayId, sf::Error> {
    let mut bytes = [0; 0x40];
    let len = 0x40.min(name.as_bytes().len());
    bytes[..len].copy_from_slice(name.as_bytes());
    let req = RequestContext::new_from_slice(&bytes).with_command(1010);

    sf::invoke_service_request(self, &req, |_, res| {
      res.try_get_ref().map(|id| DisplayId(*id, bytes))
    })
  }

  pub fn create_stray_layer(
    &self,
    display: &DisplayId,
    layer_flags: Option<u32>,
  ) -> Result<Layer, sf::Error> {
    let mut layer = Layer {
      native_window: [0; 0x100],
      layer_id: 0,
      binder_id: 0,
    };

    #[derive(Clone, Copy, AsBytes)]
    #[repr(C, packed)]
    struct Input {
      layer_flags: u32,
      _padding: u32,
      display_id: u64,
    }

    let input = Input {
      layer_flags: layer_flags.unwrap_or(0),
      _padding: 0,
      display_id: display.0,
    };

    let req = RequestContext::new_from(&input)
      .with_command(2030)
      // .cause_crash_on_request()
      .add_out_map_alias(crate::ipc::hipc::Mode::Normal, &mut layer.native_window);

    let output = sf::invoke_service_request(self, &req, |_, res| {
      #[derive(FromBytes, FromZeroes, Clone)]
      #[repr(C)]
      struct Output {
        layer_id: u64,
        native_window_size: u64,
      }

      let id: &Output = res.try_get_ref()?;

      Ok(id.clone())
    })?;

    layer.layer_id = output.layer_id;
    let size = output.native_window_size;

    let header = ParcelHeader::ref_from_prefix(&layer.native_window).unwrap();

    header
      .payload_offset
      .checked_add(header.payload_size)
      .filter(|end| *end as u64 <= size)
      .ok_or(Error::InvalidData)?;

    let binder_id = u32::from_le_bytes(
      layer.native_window[(header.payload_offset as usize)..(header.payload_offset as usize + 4)]
        .try_into()
        .unwrap(),
    );

    layer.binder_id = binder_id;

    Ok(layer)
  }

  pub fn close_stray_layer(&self, layer: Layer) -> Result<(), sf::Error> {
    let req = RequestContext::new_from(&layer.layer_id).with_command(2031);
    sf::invoke_service_request(self, &req, |_, _| Ok(()))
  }

  pub fn get_relay_service(&self) -> Result<Service<IHOSBinderDriver>, sf::Error> {
    let req = RequestContext::new().with_command(100);

    sf::invoke_service_request(self, &req, |_, mut res| {
      Ok(Service::new(res.get_move_handle()?))
    })
  }
}
