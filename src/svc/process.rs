use core::{arch::asm, default};

use bitfield_struct::bitfield;

use crate::util::{magic::Magic, result::ResultCode};

use super::handle::Handle;

#[derive(Debug, Default)]
#[repr(u8)]
pub enum InstructionSet {
  Aarch32,
  #[default]
  Aarch64,
}

impl InstructionSet {
  const fn into_bits(self) -> u8 { self as _ }
  const fn from_bits(value: u8) -> InstructionSet {
    if value == 0 {
      InstructionSet::Aarch32
    } else {
      InstructionSet::Aarch64
    }
  }
}

#[bitfield(u32)]
pub struct CreateProcessFlags {
  #[bits(1)]
  pub instruction_set: InstructionSet,
  #[bits(3)]
  pub address_space: u8,
  #[bits(1)]
  pub enable_debug: bool,
  #[bits(1)]
  pub enable_aslr: bool,
  #[bits(1)]
  pub is_app: bool,
  #[bits(4)]
  pub memory_region: u8,
  #[bits(1)]
  pub optimize_mem_layout: bool,
  #[bits(20)]
  padding: u32,
}

#[repr(C, packed)]
pub struct CreateProcessParams {
  pub name: Magic<12>,
  pub process_category: u32,
  pub title_id: u64,
  pub code_addr: usize,
  pub code_num_pages: u32,
  pub flags: CreateProcessFlags,
  pub system_resource_num_pages: u32,
}

pub mod capability {
  use core::ops::RangeInclusive;

  use bitfield_struct::bitfield;

  pub fn thread_info(priority_range: RangeInclusive<u8>, core_range: RangeInclusive<u8>) -> u32 {
    #[bitfield(u32)]
    struct Bitfield {
      #[bits(4)]
      pattern: u16,
      #[bits(6)]
      lowest_priority: u8,
      #[bits(6)]
      highest_priority: u8,
      #[bits(8)]
      lowest_core_id: u8,
      #[bits(8)]
      highest_core_id: u8,
    }

    Bitfield::new()
      .with_pattern(0b0111)
      .with_lowest_priority(*priority_range.start())
      .with_highest_priority(*priority_range.end())
      .with_lowest_core_id(*core_range.start())
      .with_highest_core_id(*core_range.end())
      .into_bits()
  }

  pub enum ApplicationType {
    System,
    Application,
    Applet,
  }

  pub fn program_type(value: ApplicationType) -> u32 {
    #[bitfield(u32)]
    struct Bitfield {
      #[bits(14)]
      pattern: u16,
      #[bits(3)]
      application_type: u8,
      #[bits(15)]
      padding: u32,
    }

    Bitfield::new()
      .with_pattern(0b01111111111111)
      .with_application_type(match value {
        ApplicationType::System => 0,
        ApplicationType::Application => 1,
        ApplicationType::Applet => 2,
      })
      .into_bits()
  }

  pub fn kernel_version(major_version: u16, minor_version: u8) -> u32 {
    #[bitfield(u32)]
    struct Bitfield {
      #[bits(15)]
      pattern: u16,
      #[bits(4)]
      minor_version: u8,
      #[bits(13)]
      major_version: u16,
    }

    Bitfield::new()
      .with_pattern(0b011111111111111)
      .with_major_version(major_version)
      .with_minor_version(minor_version)
      .into_bits()
  }

  pub fn handle_table_size(size: u16) -> u32 {
    #[bitfield(u32)]
    struct Bitfield {
      #[bits(16)]
      pattern: u16,
      #[bits(10)]
      handle_table_size: u16,
      #[bits(6)]
      padding: u32,
    }

    Bitfield::new()
      .with_pattern(0b0111111111111111)
      .with_handle_table_size(size)
      .into_bits()
  }

  pub fn debug_flags(allow_debug: bool, permit_forceful_debug: bool) -> u32 {
    #[bitfield(u32)]
    struct Bitfield {
      #[bits(17)]
      pattern: u32,
      #[bits(1)]
      allow_debug: bool,
      #[bits(1)]
      permit_forceful_debug: bool,
      #[bits(13)]
      padding: u32,
    }

    Bitfield::new()
      .with_pattern(0b01111111111111111)
      .with_allow_debug(allow_debug)
      .with_permit_forceful_debug(permit_forceful_debug)
      .into_bits()
  }
}

pub struct Process;

impl Handle<Process> {
  // todo: exosphere current handle using GetInfo
  pub fn current() -> Handle<Process> { Handle::new(0xFFFF8001) }

  pub fn create(
    params: CreateProcessParams,
    capabilities: &[u32],
  ) -> Result<Handle<Process>, ResultCode> {
    let handle: u32;
    let result: u32;
    unsafe {
      asm!(
        "svc #0x79",
        in("x1") (&params) as *const _,
        in("x2") capabilities.as_ptr(),
        in("x3") capabilities.len(),
        lateout("w0") result,
        lateout("w1") handle,
      )
    }

    ResultCode::as_result(result).and(Ok(Handle::new(handle)))
  }

  pub fn terminate(self) -> Result<(), ResultCode> {
    let result: u32;

    unsafe {
      asm!(
        "svc #0x7b",
        in("w0") self.value,
        lateout("w0") result
      )
    }

    ResultCode::as_result(result)
  }
}
