use core::mem::size_of;
use zerocopy::little_endian::{I64, U16, U32, U64};
use zerocopy::macro_util::transmute_ref;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};
use crate::Mod0;
use crate::module::{get_global_ptr, transmute_offset};
use crate::reloc::DynTags::Unknown;

#[derive(FromBytes, FromZeroes, AsBytes)]
pub enum RelocationType {
    AArch64Abs64,
    AArch64GlobDat,
    AArch64JumpSlot,
    AArch64Relative,
    Unknown(u32)
}

impl From<u32> for RelocationType {
    fn from(value: u32) -> Self {
        match (value) {
            257 => RelocationType::AArch64Abs64,
            1025 => RelocationType::AArch64GlobDat,
            1026 => RelocationType::AArch64JumpSlot,
            1027 => RelocationType::AArch64Relative,
            v => RelocationType::Unknown(v)
        }
    }
}

#[repr(C)]
#[derive(FromBytes, FromZeroes, AsBytes)]
pub struct InfoSymbol {
    pub relocation_type: U32,
    pub symbol: U32,
}

#[repr(C)]
#[derive(FromBytes, FromZeroes, AsBytes)]
pub struct RelocationEntry {
    #[cfg(target_pointer_width = "64")]
    pub offset: U64,
    #[cfg(target_pointer_width = "32")]
    pub offset: U32,
    #[cfg(target_pointer_width = "16")]
    pub offset: U16,
    pub info: InfoSymbol,
}

#[repr(C)]
#[derive(FromBytes, FromZeroes, AsBytes)]
pub struct RelocationAddendEntry {
    #[cfg(target_pointer_width = "64")]
    pub offset: U64,
    #[cfg(target_pointer_width = "32")]
    pub offset: U32,
    #[cfg(target_pointer_width = "16")]
    pub offset: U16,
    pub info: InfoSymbol,
    pub addend: I64,
}

enum DynTags {
    Null,
    RelocationAddendOffset,
    RelocationAddendEntrySize,
    RelocationAddendCount,
    RelocationOffset,
    // RelocationSize,
    RelocationEntrySize,
    RelocationCount,

    Unknown(u32),
}

impl From<u32> for DynTags {
    fn from(value: u32) -> Self {
        match value {
            0 => DynTags::Null,
            7 => DynTags::RelocationAddendOffset,
            9 => DynTags::RelocationAddendEntrySize,
            0x6FFFFFF9 => DynTags::RelocationAddendCount,
            17 => DynTags::RelocationOffset,
            19 => DynTags::RelocationEntrySize,
            0x6FFFFFFA => DynTags::RelocationCount,

            v => Unknown(v)
        }
    }
}

#[derive(Default)]
struct TemporaryRelocationInfo {
    offset: Option<usize>,
    entry_size: Option<usize>,
    count: Option<usize>,
}

struct RelocationInfo {
    offset: usize,
    entry_size: usize,
    count: usize,
}

impl RelocationInfo {
    fn load_from_temporary<EntryType>(temp: &TemporaryRelocationInfo) -> Option<RelocationInfo> {
        Some(RelocationInfo {
            offset: temp.offset?,
            entry_size: temp.entry_size.unwrap_or(size_of::<EntryType>()),
            count: temp.count?,
        })
    }

    fn entries<EntryType>(&self) -> &'static [EntryType] {
        unsafe {
            core::slice::from_raw_parts(get_global_ptr(self.offset) as *const EntryType, self.count)
        }
    }
}

#[derive(FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct ElfDyn {
    tag: U32,
    #[cfg(target_pointer_width = "64")]
    pub value: U64,
    #[cfg(target_pointer_width = "32")]
    pub value: U32,
    #[cfg(target_pointer_width = "16")]
    pub value: U16,
}

pub unsafe fn relocate_self(mod0: &Mod0) {
    let mut current_elf_dyn_ptr = get_global_ptr(mod0.dynamic_offset.get() as usize);

    let mut temp_relocation_info = TemporaryRelocationInfo::default();
    let mut temp_relocation_addend_info = TemporaryRelocationInfo::default();

    loop {
        let current_elf_dyn: &'static ElfDyn = transmute_ref(&*current_elf_dyn_ptr);

        match DynTags::from(current_elf_dyn.tag.get()) {
            DynTags::Null => break,
            DynTags::RelocationOffset => temp_relocation_info.offset = Some(current_elf_dyn.value.get() as usize),
            DynTags::RelocationCount => temp_relocation_info.count = Some(current_elf_dyn.value.get() as usize),
            DynTags::RelocationEntrySize => temp_relocation_info.entry_size = Some(current_elf_dyn.value.get() as usize),
            DynTags::RelocationAddendOffset => temp_relocation_addend_info.offset = Some(current_elf_dyn.value.get() as usize),
            DynTags::RelocationAddendCount => temp_relocation_addend_info.count = Some(current_elf_dyn.value.get() as usize),
            DynTags::RelocationAddendEntrySize => temp_relocation_addend_info.entry_size = Some(current_elf_dyn.value.get() as usize),

            Unknown(_) => {}
        }

        current_elf_dyn_ptr = current_elf_dyn_ptr.add(size_of::<ElfDyn>());
    }

    if let Some(relocation_info) = RelocationInfo::load_from_temporary::<RelocationEntry>(&temp_relocation_info) {
        for relocation in relocation_info.entries() {
            relocation
        }
    }

    if let Some(relocation_addend_info) = RelocationInfo::load_from_temporary::<RelocationAddendEntry>(&temp_relocation_addend_info) {}
}