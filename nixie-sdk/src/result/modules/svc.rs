use crate::result::{result_code::ResultCode, result_code_modules::ResultCodeDescription};

pub const OUT_OF_SESSIONS: ResultCode = ResultCode::of(1, 7);
pub const INVALID_ARGUMENT: ResultCode = ResultCode::of(1, 14);
pub const NOT_IMPLEMENTED: ResultCode = ResultCode::of(1, 33);
pub const STOP_PROCESSING_EXCEPTION: ResultCode = ResultCode::of(1, 54);
pub const NO_SYNCHRONIZATION_OBJECT: ResultCode = ResultCode::of(1, 57);
pub const TERMINATION_REQUESTED: ResultCode = ResultCode::of(1, 59);
pub const NO_EVENT: ResultCode = ResultCode::of(1, 70);
pub const INVALID_SIZE: ResultCode = ResultCode::of(1, 101);
pub const INVALID_ADDRESS: ResultCode = ResultCode::of(1, 102);
pub const OUT_OF_RESOURCE: ResultCode = ResultCode::of(1, 103);
pub const OUT_OF_MEMORY: ResultCode = ResultCode::of(1, 104);
pub const OUT_OF_HANDLES: ResultCode = ResultCode::of(1, 105);
pub const INVALID_CURRENT_MEMORY: ResultCode = ResultCode::of(1, 106);
pub const INVALID_NEW_MEMORY_PERMISSION: ResultCode = ResultCode::of(1, 108);
pub const INVALID_MEMORY_REGION: ResultCode = ResultCode::of(1, 110);
pub const INVALID_PRIORITY: ResultCode = ResultCode::of(1, 112);
pub const INVALID_CORE_ID: ResultCode = ResultCode::of(1, 113);
pub const INVALID_HANDLE: ResultCode = ResultCode::of(1, 114);
pub const INVALID_POINTER: ResultCode = ResultCode::of(1, 115);
pub const INVALID_COMBINATION: ResultCode = ResultCode::of(1, 116);
pub const TIMED_OUT: ResultCode = ResultCode::of(1, 117);
pub const CANCELLED: ResultCode = ResultCode::of(1, 118);
pub const OUT_OF_RANGE: ResultCode = ResultCode::of(1, 119);
pub const INVALID_ENUM_VALUE: ResultCode = ResultCode::of(1, 120);
pub const NOT_FOUND: ResultCode = ResultCode::of(1, 121);
pub const BUSY: ResultCode = ResultCode::of(1, 122);
pub const SESSION_CLOSED: ResultCode = ResultCode::of(1, 123);

pub fn get_result_description(code: u32) -> Option<ResultCodeDescription> {
  match code {
    7   => Some(ResultCodeDescription { name: "out_of_sessions",           description: "Handle<Port> has a maximum number of sessions. This error is sent when you exceed it", namespace: None }),
    14  => Some(ResultCodeDescription { name: "invalid_argument",          description: "A generic error sent when you provide an invalid argument", namespace: None }),
    33  => Some(ResultCodeDescription { name: "not_implemented",           description: "This SVC is unimplemented, or is disabled on your hardware", namespace: None }),
    54  => Some(ResultCodeDescription { name: "stop_processing_exception", description: "If the debug target is dying, the kernel cannot process the exception", namespace: None }),
    // TODO: 57 - NoSynchronizationObject
    59  => Some(ResultCodeDescription { name: "termination_requested",     description: "If the target thread is dying, the kernel cannot process the request", namespace: None }),
    

    123 => Some(ResultCodeDescription { name: "session_closed",            description: "The Handle<ServerSession> or Handle<ClientSession> is closed", namespace: None }),

    _ => None
  }
}