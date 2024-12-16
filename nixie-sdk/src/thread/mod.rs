use core::mem::ManuallyDrop;

use zerocopy::{native_endian::{U16, U32, U64}, FromZeroes};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::svc::Handle;

pub mod local_region;

#[derive(FromBytes, AsBytes, FromZeroes)]
#[repr(C)]
struct ThreadVersion0 {
  all_threads_node: [U64; 2],
  multi_wait_object_list: [U64; 2],
  padding_0: [u8; 32],
  state: u8,
  // bool
  stack_is_aliased: u8,
  // bool
  auto_registered: u8,
  suspend_count: u8,
  base_priority: U16,
  version: U16,
  original_stack: U64,
  stack: U64,
  stack_size: U64,
  argument: U64,
  thread_function: U64,
  current_fiber: U64,
  initial_fiber: U64,

  lock_history: U32,
  padding_1: [u8; 4],
  tls_value_array: [u8; 256],
  name_buffer: [u8; 32],
  name_pointer: U64,
  cs_thread: U32,
  cv_thread: U32,
  handle: Handle<Thread>,
  padding_2: [u8; 4],
}

#[derive(FromBytes, AsBytes, FromZeroes)]
#[repr(C)]
struct ThreadVersion1 {
  all_threads_node: [U64; 2],
  multi_wait_object_list: [U64; 2],
  padding_0: [u8; 32],
  state: u8,
  // bool
  stack_is_aliased: u8,
  // bool
  auto_registered: u8,
  suspend_count: u8,
  base_priority: U16,
  version: U16,
  original_stack: U64,
  stack: U64,
  stack_size: U64,
  argument: U64,
  thread_function: U64,
  current_fiber: U64,
  initial_fiber: U64,

  tls_value_array: [u8; 256],
  name_buffer: [u8; 32],
  name_pointer: U64,
  cs_thread: U32,
  cv_thread: U32,
  handle: Handle<Thread>,
  lock_history: U32,
  thread_id: U64,
}

pub enum State {
  NotInitialized,
  Initialized,
  DestroyedBeforeStarted,
  Started,
  Terminated,
}

impl State {
  pub fn from_u8(val: u8) -> Option<State> {
    match val {
      0 => Some(State::NotInitialized),
      1 => Some(State::Initialized),
      2 => Some(State::DestroyedBeforeStarted),
      3 => Some(State::Started),
      4 => Some(State::Terminated),
       
      _ => None,
    }
  }

  pub fn to_u8(&self) -> u8 {
    match self {
      State::NotInitialized => 0,
      State::Initialized => 1,
      State::DestroyedBeforeStarted => 2,
      State::Started => 3,
      State::Terminated => 4,
    }
  }
}

impl Into<u8> for State {
  fn into(self) -> u8 {
    self.to_u8()
  }
}

#[derive(FromBytes, AsBytes, FromZeroes)]
#[repr(C)]
union Thread {
  v1: ManuallyDrop<ThreadVersion1>,
  v0: ManuallyDrop<ThreadVersion0>,
}

impl Drop for Thread {
  fn drop(&mut self) {
    
  }
}

impl Thread {
  // pub (crate) fn initialize_v1_with(handle: Handle<Thread>, thread_local_region_ptr: *mut [u8; 0x200]) -> ThreadVersion1 {
  //   ThreadVersion1 {
  //     all_threads_node: [U64::new(0); 2],
  //     multi_wait_object_list: [U64::new(0); 2],
  //     state: State::Started.into(),
      
  //   }
  // }

  unsafe fn get_v0(&self) -> &ThreadVersion0 { unsafe { &self.v0 } }
  unsafe fn get_v1(&self) -> &ThreadVersion1 { unsafe { &self.v1 } }

  pub fn get_handle(&self) -> &Handle<Thread> {
    match unsafe { self.get_v0() }.version.get() {
      0 => &unsafe { self.get_v0() }.handle,
      1 => &unsafe { self.get_v1() }.handle,

      version => panic!("Unknown thread version {version}")
    }
  }
}
