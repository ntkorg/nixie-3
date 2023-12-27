mod r#in;
mod out;
mod domain_in;
mod domain_out;

pub use r#in::*;
pub use out::*;
pub use domain_in::*;
pub use domain_out::*;

#[repr(C)]
pub enum CommandType {
    Invalid = 0,
    LegacyRequest = 1,
    Close = 2,
    LegacyControl = 3,
    Request = 4,
    Control = 5,
    RequestWithContext = 6,
    ControlWithContext = 7,
}

impl From<CommandType> for u16 {
    fn from(command_type: CommandType) -> Self {
        match command_type {
            CommandType::Invalid => 0,
            CommandType::LegacyRequest => 1,
            CommandType::Close => 2,
            CommandType::LegacyControl => 3,
            CommandType::Request => 4,
            CommandType::Control => 5,
            CommandType::RequestWithContext => 6,
            CommandType::ControlWithContext => 7,
        }
    }
}

impl From<u16> for CommandType {
    fn from(command_type: u16) -> Self {
        match command_type {
            0 => CommandType::Invalid,
            1 => CommandType::LegacyRequest,
            2 => CommandType::Close,
            3 => CommandType::LegacyControl,
            4 => CommandType::Request,
            5 => CommandType::Control,
            6 => CommandType::RequestWithContext,
            7 => CommandType::ControlWithContext,
            _ => panic!("Invalid CommandType: {}", command_type),
        }
    }
}
