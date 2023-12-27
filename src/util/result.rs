use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct ResultCode {
    value: u32,
}

pub const RESULT_SUCCESS: ResultCode = ResultCode::new(0);

impl ResultCode {
    pub const fn new(value: u32) -> Self { Self { value } }
    pub const fn as_result(result: u32) -> Result<(), Self> {
        if result == 0 {
            Ok(())
        } else {
            Err(Self::new(result))
        }
    }

    pub const fn value(&self) -> u32 { self.value }

    pub const fn module(&self) -> u32 { self.value >> 9 }
    pub const fn description(&self) -> u32 { self.value & 0x1FF }
    pub const fn is_success(&self) -> bool { self.value == 0 }
    pub const fn is_failure(&self) -> bool { !self.is_success() }
}
