use core::time::Duration;

use crate::{
    ipc::sf::{self, service::Service, Error, RequestContext},
    svc::{sleep_thread, Process},
};

use super::{ServiceRoot, sm::ServiceName};

pub struct ApplicationManagerOE;

impl ServiceRoot for ApplicationManagerOE {
    fn name() -> ServiceName { b"appletOE".into() }
    fn is_domain() -> bool { true }
}

impl Service<ApplicationManagerOE> {
    pub fn open_application_proxy(&self) -> Result<Service<ApplicationProxy>, Error> {
        let placeholder: usize = 0;
        let req = RequestContext::new_from(&placeholder)
            .with_command(0)
            .add_copy_handle(Process::current_process());

        sf::invoke_service_request(self, &req, |_, mut res| {
            Ok(Service::new(res.get_move_handle()?))
        })
    }
}

pub struct ApplicationProxy;

impl Service<ApplicationProxy> {
    pub fn get_self_controller(&self) -> Result<Service<SelfController>, Error> {
        let req = RequestContext::new().with_command(1);

        sf::invoke_service_request(self, &req, |_, mut res| {
            Ok(self.subservice(res.get_object()?))
        })
    }
}

pub struct SelfController;

impl Service<SelfController> {
    pub fn exit(&self) -> Result<(), Error> {
        let req = RequestContext::new().with_command(0);

        sf::invoke_service_request(self, &req, |_, _| loop {
            sleep_thread(Duration::from_secs(0x1000));
        })
    }
}
