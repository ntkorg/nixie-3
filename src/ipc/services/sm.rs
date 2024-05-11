use crate::ipc::sf::{self, service::Service};
use crate::ipc::sf::{Error, RequestContext, Response};
use crate::svc::{self, output_debug_string};
use crate::util::magic::{reverse_magic, Magic};
use crate::util::result::ResultCode;

pub type ServiceName = Magic<8>;

pub struct ServiceManager;

impl ServiceManager {
  pub fn connect() -> Result<Service<Self>, ResultCode> {
    let session = svc::Handle::<svc::Session>::connect_to_named_port(b"sm:")?;
    Ok(Service::new(session))
  }
}

impl Service<ServiceManager> {
  pub fn register_client(&self) -> Result<(), Error> {
    let placeholder: usize = 0;
    let req = RequestContext::new_from(&placeholder)
      .set_send_pid()
      .with_command(0);

    sf::invoke_service_request(self, &req, |_, _| Ok(()))
  }

  pub fn get_service_handle(&self, name: ServiceName) -> Result<svc::Handle<svc::Session>, Error> {
    let req = RequestContext::new_from(&name).with_command(1);

    output_debug_string("waow");

    sf::invoke_service_request(self, &req, |_, mut res| {
      output_debug_string("gay");

      Ok(res.get_move_handle().unwrap_or_else(|_| {
        output_debug_string("failed to get handle");
        unreachable!("kill yourself");
      }))
    })
  }
}
