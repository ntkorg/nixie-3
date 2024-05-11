use core::arch::asm;

use heapless::String;
use zerocopy::macro_util::{transmute_mut, transmute_ref};

use crate::svc::output_debug_string;

use super::{reader::Reader, writer::Writer};

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
  unsafe { Writer::new(core::slice::from_raw_parts_mut(get_mut(offset), size)) }
}

pub fn get_reader<'a>(offset: usize, size: usize) -> Reader<'a> {
  unsafe { Reader::new(core::slice::from_raw_parts(get(offset), size)) }
}

pub fn dump() {
  let mut str: String<0x310> = String::new();
  let ptr = get_mut(0);
  const HEX: &'static [u8; 16] = b"0123456789abcdef";
  for i in 0..0x100 {
    if i % 16 == 0 {
      str.push('\n').unwrap();
    }
    let byte = unsafe { *ptr.add(i) };
    str.push(HEX[((byte >> 4) & 0xF) as usize] as char).unwrap();
    str.push(HEX[((byte >> 0) & 0xF) as usize] as char).unwrap();
    str.push(' ').unwrap();
  }

  output_debug_string(str.as_str());
}
