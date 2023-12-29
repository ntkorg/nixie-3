use crate::util::magic::MagicU32;
use crate::util::result::{ResultCode, RESULT_SUCCESS};

use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Copy, Clone, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C, packed)]
pub struct OutHeader {
    magic: MagicU32,
    version: u32,
    result: ResultCode,
    token: u32,
}

impl Default for OutHeader {
    fn default() -> Self {
        Self {
            magic: MagicU32::new(*b"SFCO"),
            version: 0,
            result: RESULT_SUCCESS,
            token: 0,
        }
    }
}

impl OutHeader {
    pub fn magic(&self) -> MagicU32 { self.magic }

    pub fn version(&self) -> u32 { self.version }

    pub fn result(&self) -> ResultCode { self.result }

    pub fn token(&self) -> u32 { self.token }

    pub fn set_magic(&mut self, magic: MagicU32) { self.magic = magic; }

    pub fn set_version(&mut self, version: u32) { self.version = version; }

    pub fn set_result(&mut self, result: ResultCode) { self.result = result; }

    pub fn set_token(&mut self, token: u32) { self.token = token; }

    pub fn with_magic(mut self, magic: MagicU32) -> Self {
        self.set_magic(magic);
        self
    }

    pub fn with_version(mut self, version: u32) -> Self {
        self.set_version(version);
        self
    }

    pub fn with_result(mut self, result: ResultCode) -> Self {
        self.set_result(result);
        self
    }

    pub fn with_token(mut self, token: u32) -> Self {
        self.set_token(token);
        self
    }

    pub fn new(magic: MagicU32, version: u32, result: ResultCode, token: u32) -> Self {
        Self::default()
            .with_magic(magic)
            .with_version(version)
            .with_result(result)
            .with_token(token)
    }
}
