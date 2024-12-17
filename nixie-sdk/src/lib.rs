#![no_std]
#![feature(trivial_bounds)]
#![feature(naked_functions)]
#![feature(negative_impls)]

pub mod entrypoint;
pub mod result;
pub mod svc;
pub mod thread;
pub mod util;
pub mod nnsdk;

// extern crate alloc;