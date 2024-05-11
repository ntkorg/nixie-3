use zerocopy_derive::AsBytes;

use crate::ipc::{
  hipc::Mode,
  sf::{self, invoke_service_request, service::Service, Error, RequestContext},
};

pub struct IHOSBinderDriver;

impl Service<IHOSBinderDriver> {
  pub fn transact_parcel(
    &self,
    id: u32,
    code: u32,
    input: &[u8],
    output: &mut [u8],
    flags: u32,
  ) -> Result<(), Error> {
    self.transact_parcel_base(id, code, input, output, flags, false)
  }

  pub fn transact_parcel_auto(
    &self,
    id: u32,
    code: u32,
    input: &[u8],
    output: &mut [u8],
    flags: u32,
  ) -> Result<(), Error> {
    self.transact_parcel_base(id, code, input, output, flags, true)
  }

  fn transact_parcel_base(
    &self,
    id: u32,
    code: u32,
    input: &[u8],
    output: &mut [u8],
    flags: u32,
    auto: bool,
  ) -> Result<(), Error> {
    #[derive(Clone, Copy, AsBytes)]
    #[repr(C, packed)]
    struct Input {
      session_id: u32,
      code: u32,
      flags: u32,
    }

    let bind_input = Input {
      session_id: id,
      code,
      flags,
    };
    let req = RequestContext::new_from(&bind_input);

    let req = if auto {
      req
        .with_command(3)
        .add_in_autoselect(Mode::Normal, input)
        .add_out_autoselect(Mode::Normal, output)
    } else {
      req
        .with_command(1)
        .add_in_map_alias(Mode::Normal, input)
        .add_out_map_alias(Mode::Normal, output)
    };

    invoke_service_request(self, &req, |_, _| Ok(()))
  }
}
