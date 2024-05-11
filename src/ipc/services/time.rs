use crate::ipc::sf::{self, service::Service, Error, RequestContext};

use super::{sm::ServiceName, ServiceRoot};

pub struct TimeUser;

impl ServiceRoot for TimeUser {
  fn name() -> ServiceName { b"time:u".into() }

  fn is_domain() -> bool { false }
}

impl Service<TimeUser> {
  pub fn get_standard_user_system_clock(&self) -> Result<Service<SystemClock>, Error> {
    let req = RequestContext::new().with_command(0);

    sf::invoke_service_request(self, &req, |_, mut res| {
      Ok(self.subservice(res.get_object()?))
    })
  }
}

pub struct SystemClock;

impl Service<SystemClock> {
  pub fn get_current_time(&self) -> Result<u64, Error> {
    let req = RequestContext::new().with_command(0);

    sf::invoke_service_request(self, &req, |_, res| res.try_get_ref().map(|x| *x))
  }
}
