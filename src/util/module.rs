use core::arch::global_asm;
use zerocopy::macro_util::transmute_ref;

global_asm!(
  r#"
.global get_module_start
get_module_start:
  adrp x0, beginning
  ret
"#
);

extern "C" {
  pub fn get_module_start() -> usize;
}

pub unsafe fn get_global_ptr(offset: usize) -> *const u8 {
  let global_addr = offset + get_module_start();

  global_addr as *const u8
}

pub unsafe fn get_global_ptr_mut(offset: usize) -> *mut u8 {
  let global_addr = offset + get_module_start();

  global_addr as *mut u8
}

pub unsafe fn transmute_offset<T>(offset: usize) -> &'static T {
  transmute_ref(&*get_global_ptr(offset))
}

// pub (in super) unsafe fn transmute_offset_mut<T>(offset: usize) -> &'static mut T {
//     transmute_mut(&mut *get_global_ptr_mut(offset))
// }
