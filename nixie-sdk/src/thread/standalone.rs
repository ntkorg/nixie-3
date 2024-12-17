use core::mem::MaybeUninit;

use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::svc::Handle;

use super::Thread;

#[derive(AsBytes, FromBytes, FromZeroes)]
#[repr(C)]
pub struct StandaloneData {
    reserved_1: [u8; 0xF4], 
    contents: StandaloneThreadData,
}

#[derive(AsBytes, FromBytes, FromZeroes)]
#[repr(C)]
pub struct StandaloneThreadData {
    pub handle: Handle<crate::svc::Thread>,    
}

impl !Send for StandaloneData {}

impl StandaloneData {
    pub fn get_data(&self) -> &StandaloneThreadData {
        &self.contents
    }

    pub fn mut_data(&mut self) -> &mut StandaloneThreadData {
        &mut self.contents
    }

    pub fn set_data(&mut self, thread_data: StandaloneThreadData) {
        self.contents = thread_data;
    }
}