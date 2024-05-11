use core::{
  alloc::{GlobalAlloc, Layout},
  cell::UnsafeCell,
  ptr::null_mut,
  sync::atomic::{AtomicPtr, AtomicUsize, Ordering::SeqCst},
};

use zerocopy::ByteSliceMut;

use crate::svc;

use super::result::ResultCode;

use good_memory_allocator::SpinLockedAllocator;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

pub fn is_initialized() -> bool { ALLOCATOR.was_initialized() }
pub fn initialize(size: usize) -> Result<(), ResultCode> {
  let heap = svc::set_heap_size(size)?;

  unsafe {
    ALLOCATOR.init(heap.as_mut_ptr() as usize, heap.len());
  }
  
  Ok(())
}

// pub struct SimpleAllocator {
//   heap: Option<UnsafeCell<&'static mut [u8]>>,
//   remaining: AtomicUsize, // we allocate from the top, counting down
// }

// #[global_allocator]
// pub static mut ALLOCATOR: SimpleAllocator = SimpleAllocator {
//   heap: None,
//   remaining: AtomicUsize::new(0),
// };

// impl SimpleAllocator {
//   pub fn is_initialized(&self) -> bool {
//     self.heap.is_some()
//   }
//   pub fn initialize(&mut self, size: usize) -> Result<(), ResultCode> {
//     let heap = svc::set_heap_size(size)?;

//     self.heap = Some(UnsafeCell::new(heap));
//     self.remaining = AtomicUsize::new(size);

//     Ok(())
//   }
// }

// unsafe impl Sync for SimpleAllocator {}

// unsafe impl GlobalAlloc for SimpleAllocator {
//   unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//     if self.heap.is_none() {
//       panic!("Cannot allocate on an uninitialized allocator")
//     }

//     let size = layout.size();
//     let align = layout.align();

//     // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
//     // So we can safely use a mask to ensure alignment without worrying about UB.
//     let align_mask_to_round_down = !(align - 1);

//     let mut allocated = 0;
//     if self
//       .remaining
//       .fetch_update(SeqCst, SeqCst, |mut remaining| {
//         if size > remaining {
//             return None;
//         }
//         remaining -= size;
//         remaining &= align_mask_to_round_down;
//         allocated = remaining;
//         Some(remaining)
//       })
//       .is_err()
//     {
//       return null_mut();
//     };

//     (*(self.heap.as_ref().unwrap().get())).as_mut_ptr().add(allocated)
//   }
//   unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
// }
