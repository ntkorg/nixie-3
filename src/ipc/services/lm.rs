use crate::{ipc::{sf::{service::Service, Error, RequestContext, self}, hipc::Mode}, svc};

use super::{sm::{ServiceManager, ServiceName}, ServiceRoot};

pub struct LogManager;

impl ServiceRoot for LogManager {
    fn name() -> ServiceName { b"lm".into() }
    fn is_domain() -> bool { false }
}

impl Service<LogManager> {
    pub fn get_logger(&self) -> Result<Service<Logger>, Error> {
        let placeholder: usize = 0;
        let req = RequestContext::new_from(&placeholder)
            .set_send_pid()
            .with_command(0)
            .with_token(0);

        sf::invoke_service_request(self, &req, |_, mut res| Ok(Service::new(res.get_move_handle()?)))
    }
}

pub struct Logger;

impl Service<Logger> {
    pub fn log(&self, message: &str) -> Result<(), Error> {
        let req = RequestContext::new()
            .add_in_autoselect(Mode::Normal, message.as_bytes())
            .with_command(0);

        sf::invoke_service_request(self, &req, |_, _| Ok(()))
    }
}
