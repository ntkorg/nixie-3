use core::arch::asm;

use zerocopy::macro_util::{transmute_ref, transmute_mut};

use super::{writer::Writer, reader::Reader};

pub fn get(offset: usize) -> *const u8 {
    let ptr: *const u8;
    unsafe {
        asm!("mrs {}, tpidrro_el0", out(reg) ptr);
        ptr.add(offset)
    }
}

pub fn get_mut(offset: usize) -> *mut u8 {
    let ptr: *mut u8;
    unsafe {
        asm!("mrs {}, tpidrro_el0", out(reg) ptr);
        ptr.add(offset)
    }
}

pub fn get_writer<'a>(offset: usize, size: usize) -> Writer<'a> {
    unsafe {
        Writer::new(core::slice::from_raw_parts_mut(get_mut(offset), size))
    }
}

pub fn get_reader<'a>(offset: usize, size: usize) -> Reader<'a> {
    unsafe {
        Reader::new(core::slice::from_raw_parts(get(offset), size))
    }
}

// pub unsafe fn transmute_offset<T>(offset: usize) -> &'static T {
//     transmute_ref(&*get(offset))
// }

// pub unsafe fn transmute_offset_mut<T>(offset: usize) -> &'static mut T {
//     transmute_mut(&mut *get_mut(offset))
// }

// pub fn slice_offset_mut<'a, T>(offset: usize, size: usize) -> &'a mut [T] {
//     let ptr: *mut u8 = get_mut(offset);
//     unsafe {
//         core::slice::from_raw_parts_mut(ptr as *mut T, size)
//     }
// }

// pub fn slice_offset<'a, T>(offset: usize, size: usize) -> &'a [T] {
//     let ptr: *const u8 = get(offset);
//     unsafe {
//         core::slice::from_raw_parts(ptr as *const T, size)
//     }
// }
