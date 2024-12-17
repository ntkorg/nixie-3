use crate::reloc::DynTags::Unknown;
use core::{
  arch::{asm, global_asm},
  mem::size_of,
};
use heapless::String;
use nixie_sdk::svc;
use zerocopy::macro_util::transmute_ref;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
pub struct Mod0 {
  pub magic: u32,
  pub dynamic_offset: u32,
  pub bss_start_offset: u32,
  pub bss_end_offset: u32,
  pub eh_start_offset: u32,
  pub eh_end_offset: u32,
  pub runtime_module_offset: u32,
}

fn get_module_start() -> usize {
  let module_start: usize;

  // ASM to do this without linker getting in the way
  unsafe { asm!("adrp x0, __nixie_module_intro", out("x0") module_start) };

  return module_start;
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

#[derive(PartialEq, Eq)]
pub enum RelocationType {
  AArch64Abs64,
  AArch64GlobDat,
  AArch64JumpSlot,
  AArch64Relative,
  Unknown(u32),
}

impl From<u32> for RelocationType {
  fn from(value: u32) -> Self {
    match value {
      257 => RelocationType::AArch64Abs64,
      1025 => RelocationType::AArch64GlobDat,
      1026 => RelocationType::AArch64JumpSlot,
      1027 => RelocationType::AArch64Relative,
      v => RelocationType::Unknown(v),
    }
  }
}

#[repr(C)]
#[derive(FromBytes, FromZeroes, AsBytes)]
pub struct InfoSymbol {
  pub relocation_type: u32,
  pub symbol: u32,
}

#[repr(C)]
#[derive(FromBytes, FromZeroes, AsBytes)]
pub struct RelocationEntry {
  #[cfg(target_pointer_width = "64")]
  pub offset: u64,
  #[cfg(target_pointer_width = "32")]
  pub offset: u32,
  pub info: InfoSymbol,
}

#[repr(C)]
#[derive(FromBytes, FromZeroes, AsBytes)]
pub struct RelocationAddendEntry {
  #[cfg(target_pointer_width = "64")]
  pub offset: u64,
  #[cfg(target_pointer_width = "32")]
  pub offset: u32,
  pub info: InfoSymbol,
  pub addend: u64,
}

enum DynTags {
  Null,
  RelocationAddendOffset,
  // RelocationAddendEntrySize,
  RelocationAddendCount,
  RelocationOffset,
  // RelocationEntrySize,
  RelocationCount,

  Unknown(u32),
}

impl From<u32> for DynTags {
  fn from(value: u32) -> Self {
    match value {
      0 => DynTags::Null,
      7 => DynTags::RelocationAddendOffset,
      // 9 => DynTags::RelocationAddendEntrySize,
      0x6FFFFFF9 => DynTags::RelocationAddendCount,
      17 => DynTags::RelocationOffset,
      // 19 => DynTags::RelocationEntrySize,
      0x6FFFFFFA => DynTags::RelocationCount,

      v => Unknown(v),
    }
  }
}

#[derive(Default)]
struct TemporaryRelocationInfo {
  offset: Option<usize>,
  // entry_size: Option<usize>,
  count: Option<usize>,
}

struct RelocationInfo {
  offset: usize,
  // entry_size: usize,
  count: usize,
}

impl RelocationInfo {
  fn load_from_temporary<EntryType>(temp: &TemporaryRelocationInfo) -> Option<RelocationInfo> {
    Some(RelocationInfo {
      offset: temp.offset?,
      // entry_size: temp.entry_size.unwrap_or(size_of::<EntryType>()),
      count: temp.count?,
    })
  }

  fn entries<EntryType>(&self) -> &'static [EntryType] {
    unsafe {
      core::slice::from_raw_parts(get_global_ptr(self.offset) as *const EntryType, self.count)
    }
  }
}

#[derive(FromBytes, FromZeroes)]
#[repr(C)]
pub struct ElfDyn {
  tag: u32,
  #[cfg(target_pointer_width = "64")]
  pub value: u64,
  #[cfg(target_pointer_width = "32")]
  pub value: u32,
}

pub unsafe fn relocate_self(dynamic_offset: u32) {
  let mut current_elf_dyn_ptr = get_global_ptr(dynamic_offset as usize);

  let mut temp_relocation_info = TemporaryRelocationInfo::default();
  let mut temp_relocation_addend_info = TemporaryRelocationInfo::default();

  loop {
    let current_elf_dyn: &'static ElfDyn = unsafe { transmute_ref(&*current_elf_dyn_ptr) };

    match DynTags::from(current_elf_dyn.tag) {
      DynTags::Null => break,
      DynTags::RelocationOffset => {
        temp_relocation_info.offset = Some(current_elf_dyn.value as usize)
      }
      DynTags::RelocationCount => temp_relocation_info.count = Some(current_elf_dyn.value as usize),
      // DynTags::RelocationEntrySize => temp_relocation_info.entry_size = Some(current_elf_dyn.value as usize),
      DynTags::RelocationAddendOffset => {
        temp_relocation_addend_info.offset = Some(current_elf_dyn.value as usize)
      }
      DynTags::RelocationAddendCount => {
        temp_relocation_addend_info.count = Some(current_elf_dyn.value as usize)
      }
      // DynTags::RelocationAddendEntrySize => temp_relocation_addend_info.entry_size = Some(current_elf_dyn.value as usize),
      Unknown(_) => {}
    }

    current_elf_dyn_ptr = current_elf_dyn_ptr.add(size_of::<ElfDyn>());
  }

  if let Some(relocation_info) =
    RelocationInfo::load_from_temporary::<RelocationEntry>(&temp_relocation_info)
  {
    for relocation in relocation_info.entries::<RelocationEntry>() {
      if RelocationType::AArch64Relative == RelocationType::from(relocation.info.relocation_type) {
        *(get_global_ptr_mut(relocation.offset as usize) as *mut usize) += get_module_start();
      }
    }
  }

  if let Some(relocation_addend_info) =
    RelocationInfo::load_from_temporary::<RelocationAddendEntry>(&temp_relocation_addend_info)
  {
    for relocation in relocation_addend_info.entries::<RelocationAddendEntry>() {
      if RelocationType::AArch64Relative == RelocationType::from(relocation.info.relocation_type) {
        *(get_global_ptr_mut(relocation.offset as usize) as *mut usize) =
          get_module_start() + relocation.addend as usize;
      }
    }
  }
}
