use self::sm::{ServiceManager, ServiceName};

use super::sf::{service::Service, Error};

pub mod am;
pub mod lm;
pub mod sm;

pub trait ServiceRoot: Sized {
    fn name() -> ServiceName;
    fn is_domain() -> bool;
    fn open(sm: &Service<ServiceManager>) -> Result<Service<Self>, Error> {
        let session = sm.get_service_handle(Self::name().into())?;

        if Self::is_domain() {
            Ok(Service::new(session).convert_to_domain()?)
        } else {
            Ok(Service::new(session))
        }
    }
}
