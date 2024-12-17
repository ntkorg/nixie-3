pub mod arbitrate_lock;
pub mod arbitrate_unlock;
pub mod r#break;
pub mod cancel_synchronization;
pub mod clear_event;
pub mod close_handle;
pub mod connect_to_named_port;
pub mod create_thread;
pub mod create_transfer_memory;
pub mod exit_process;
pub mod exit_thread;
pub mod flush_data_cache;
pub mod flush_entire_data_cache;
pub mod get_current_processor_number;
pub mod get_info;
pub mod get_process_id;
pub mod get_system_tick;
pub mod get_thread_core_mask;
pub mod get_thread_id;
pub mod get_thread_priority;
pub mod map_memory;
pub mod map_physical_memory;
pub mod map_shared_memory;
pub mod output_debug_string;
pub mod query_memory;
pub mod reset_signal;
pub mod return_from_exception;
pub mod send_async_request_with_user_buffer;
pub mod send_sync_request;
pub mod send_sync_request_light;
pub mod send_sync_request_with_user_buffer;
pub mod set_heap_size;
pub mod set_memory_attribute;
pub mod set_memory_permission;
pub mod set_thread_core_mask;
pub mod set_thread_priority;
pub mod signal_event;
pub mod signal_process_wide_key;
pub mod sleep_thread;
pub mod start_thread;
pub mod unmap_memory;
pub mod unmap_physical_memory;
pub mod unmap_shared_memory;
pub mod wait_process_wide_key_atomic;
pub mod wait_synchronization;
// TODO: GetDebugFutureThreadInfo (x2E)
// TODO: GetLastThreadInfo (x2F)
pub mod get_resource_limit_current_value;
pub mod get_resource_limit_limit_value;
pub mod get_resource_limit_peak_value;
pub mod get_thread_context3;
pub mod set_thread_activity;
pub mod signal_to_address;
pub mod synchronize_preemption_state;
pub mod wait_for_address;
// DOES NOT EXIST
// TODO: CreateIoPool (0x39)
// TODO: CreateIoRegion (0x3A)
// DOES NOT EXIST
// TODO: KernelDebug (0x3C)
// TODO: ChangeKernelTraceState (0x3D)
// DOES NOT EXIST
// DOES NOT EXIST
pub mod accept_session;
pub mod create_event;
pub mod create_session;
pub mod reply_and_receive;
pub mod reply_and_receive_light;
pub mod reply_and_receive_with_user_buffer;
// TODO: MapIoRegion (0x46)
// TODO: UnmapIoRegion (0x47)
pub mod control_code_memory;
pub mod create_code_memory;
pub mod map_physical_memory_unsafe;
pub mod set_unsafe_limit;
pub mod sleep_system;
pub mod unmap_physical_memory_unsafe;
// TODO: ReadWriteRegister (0x4E)
pub mod create_shared_memory;
pub mod map_transfer_memory;
pub mod set_process_activity;
pub mod unmap_transfer_memory;
// TODO: CreateInterruptEvent (0x53)
pub mod query_physical_address;
// TODO: QueryMemoryMapping (0x55)
// TODO: CreateDeviceAddressSpace (0x56)
// TODO: AttachDeviceAddressSpace (0x57)
// TODO: DetachDeviceAddressSpace (0x58)
// TODO: MapDeviceAddressSpaceByForce (0x59)
// TODO: MapDeviceAddressSpaceAligned (0x5A)
// TODO: MapDeviceAddressSpace (0x5B)
// TODO: UnmapDeviceAddressSpace (0x5C)
pub mod break_debug_process;
pub mod continue_debug_event;
pub mod debug_active_process;
pub mod flush_process_data_cache;
pub mod get_debug_event;
pub mod get_process_list;
pub mod get_thread_list;
pub mod invalidate_process_data_cache;
pub mod store_process_data_cache;
pub mod terminate_debug_process;
// TODO: GetDebugThreadContext (0x67)
// TODO: SetDebugThreadContext (0x68)
// TODO: QueryDebugProcessMemory (0x69)
// TODO: ReadDebugProcessMemory (0x6A)
// TODO: WriteDebugProcessMemory (0x6B)
// TODO: SetHardwareBreakpoint (0x6C)
// TODO: GetDebugThreadParam (0x6D)
// DOES NOT EXIST
pub mod call_secure_monitor;
pub mod connect_to_port;
pub mod create_port;
pub mod create_process;
pub mod create_resource_limit;
pub mod get_process_info;
pub mod get_system_info;
pub mod manage_named_port;
pub mod map_process_code_memory;
pub mod map_process_memory;
pub mod query_process_memory;
pub mod set_process_memory_permission;
pub mod set_resource_limit_value;
pub mod start_process;
pub mod terminate_process;
pub mod unmap_process_code_memory;
pub mod unmap_process_memory;

// Types
use crate::{result::result_code::ResultCode, util::USIZE};
use bitfield_struct::bitfield;
use core::{any::type_name, ffi::c_void, fmt::Write};
use zerocopy::native_endian::{U128, U32, U64};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[bitfield(u32)]
pub struct MemoryPermission {
  pub read: bool,
  pub write: bool,
  pub execute: bool,

  #[bits(24)]
  padding_0: u32,

  pub dont_care: bool,

  #[bits(4)]
  padding_1: u32,
}

pub enum MemoryType {
  Free,
  Io,
  Static,
  Code,
  CodeData,
  Normal,
  Shared,
  Alias,
  AliasCode,
  AliasCodeData,
  Ipc,
  Stack,
  ThreadLocal,
  Transferred,
  SharedTransferred,
  SharedCode,
  Inaccessible,
  NonSecureIpc,
  NonDeviceIpc,
  Kernel,
  GeneratedCode,
  CodeOut,
  Coverage,
  Insecure,
}

#[bitfield(u32)]
pub struct MemoryAttribute {
  locked: bool,
  ipc_locked: bool,
  device_shared: bool,
  uncached: bool,

  #[bits(28)]
  unused: u32,
}

#[derive(FromBytes, AsBytes, FromZeroes)]
#[repr(C)]
struct RawMemoryInfo {
  address: U64,
  size: U64,
  type_int: U32,
  memory_attribute_int: U32,
  memory_permission_int: U32,
  ipc_ref_count: U32,
  device_ref_count: U32,
  padding: U32,
}

pub struct MemoryInfo {
  pub address: u64,
  pub size: u64,
  type_int: u32,
  pub memory_attribute: MemoryAttribute,
  pub memory_permission: MemoryPermission,
  pub ipc_ref_count: u32,
  pub device_ref_count: u32,
}

impl MemoryInfo {
  fn from_raw(raw: RawMemoryInfo) -> MemoryInfo {
    MemoryInfo {
      address: raw.address.get(),
      size: raw.size.get(),
      type_int: raw.type_int.get(),
      memory_attribute: MemoryAttribute::from_bits(raw.memory_attribute_int.get()),
      memory_permission: MemoryPermission::from_bits(raw.memory_permission_int.get()),
      ipc_ref_count: raw.ipc_ref_count.get(),
      device_ref_count: raw.device_ref_count.get(),
    }
  }

  pub fn get_type() -> MemoryType {
    todo!("Requires some method of getting current system version");
  }
}

pub enum ThreadActivity {
  Inactive = 0,
  Active = 1,
}

pub enum ProcessActivity {
  Inactive = 0,
  Active = 1,
}

pub struct Thread;
pub struct WritableEvent;
pub struct ReadableEvent;
pub struct TransferMemory;
pub struct SharedMemory;
pub struct CodeMemory;
pub struct DeviceAddressSpace;
pub struct Process;
pub struct ResourceLimit;
pub struct ServerSession;
pub struct ClientSession;
pub struct Debug;
pub struct ClientPort;
pub struct ServerPort;

#[derive(FromBytes, AsBytes, FromZeroes)]
#[repr(transparent)]
pub struct Handle<T: ?Sized> {
  handle_index: U32,
  _marker: core::marker::PhantomData<T>,
}

impl<T: ?Sized> Handle<T> {
  pub(crate) unsafe fn from_bits(bits: u32) -> Handle<T> {
    Handle::<T> {
      handle_index: U32::new(bits),
      _marker: core::marker::PhantomData::<T> {},
    }
  }

  pub(crate) fn as_bits(&self) -> u32 { self.handle_index.get() }

  pub unsafe fn clone(&self) -> Handle<T> { Handle::<T>::from_bits(self.handle_index.get()) }
}

impl<T: ?Sized> Drop for Handle<T> {
  fn drop(&mut self) {
    // ignore the result code generated by close_handle
    let _ = unsafe { crate::svc::close_handle::close_handle(self) };
  }
}

impl<T> core::fmt::Debug for Handle<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let type_name = type_name::<T>();

    f.write_str("Handle<")?;

    if type_name.starts_with("nixie_sdk::svc::") {
      f.write_str(&type_name[16..])?;
    } else {
      f.write_str(&type_name)?;
    }

    f.write_str("> { index: 0x")?;
    
    for i in 0..8 {
      let nibble = (self.handle_index.get() >> (i * 4)) & 0b1111;

      match nibble {
        0 => f.write_char('0')?,
        1 => f.write_char('1')?,
        2 => f.write_char('2')?,
        3 => f.write_char('3')?,
        4 => f.write_char('4')?,
        5 => f.write_char('5')?,
        6 => f.write_char('6')?,
        7 => f.write_char('7')?,
        8 => f.write_char('8')?,
        9 => f.write_char('9')?,
        10 => f.write_char('A')?,
        11 => f.write_char('B')?,
        12 => f.write_char('C')?,
        13 => f.write_char('D')?,
        14 => f.write_char('E')?,
        15 => f.write_char('F')?,

        _ => unreachable!(),
      };
    }

    f.write_str(" }")?;

    Ok(())
  }
}

#[derive(FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct ThreadContext {
  pub registers: [U64; 29],
  pub frame_pointer: U64,
  pub link_register: U64,
  pub stack_pointer: U64,
  pub program_counter: U64,
  pub processor_state: U32,
  padding: U32,
  pub vectors: [U128; 32],
  pub floating_point_control_register: U32,
  pub floating_point_status_register: U32,
  pub thread_local_region_pointer: U64,
}

pub trait Event {}
impl Event for ReadableEvent {}
impl Event for WritableEvent {}

pub trait Signal {}
impl Signal for ReadableEvent {}
impl Signal for Process {}

pub trait Session {}
impl Session for ClientSession {}
impl Session for ServerSession {}

pub trait Resettable {}
impl Resettable for ReadableEvent {}
impl Resettable for Process {}

pub trait Port {}
impl Port for ClientPort {}
impl Port for ServerPort {}

pub trait Waitable {}
impl Waitable for Debug {}
impl Waitable for ClientPort {}
impl Waitable for Process {}
impl Waitable for ReadableEvent {}
impl Waitable for ServerPort {}
impl Waitable for ServerSession {}
impl Waitable for Thread {}

pub enum ArbitrationType {
  WaitIfLessThan = 0x00,
  DecrementAndWaitIfLessThan = 0x01,
  WaitIfEqual = 0x02,
}

pub enum ReplyAndReceiveError<'a> {
  PortRemoteDead(&'a Handle<ServerSession>),
  GenericResultCode(ResultCode),
}

#[derive(Eq, PartialEq)]
pub enum SlaveMemoryPermissions {
  ReadOnly,
  ReadExecute,
}

pub struct PhysicalMemoryInfo {
  pub address: u64,
  pub base_address: u64,
  pub size: u64,
}

#[derive(FromBytes, AsBytes, FromZeroes)]
#[repr(C)]
struct DebugEventInfoRaw {
  pub event_type: U32,
  pub flags: U32,
  pub thread_id: U64,
  pub per_type_specifics: DebugEventInfoPerTypeSpecificsRaw,
}

#[derive(FromBytes, AsBytes, FromZeroes)]
#[repr(C)]
union DebugEventInfoPerTypeSpecificsRaw {
  process: DebugEventInfoProcessSpecificsRaw,
  thread: DebugEventInfoThreadSpecificsRaw,
  exit: DebugEventInfoExitSpecificsRaw,
  exception: DebugEventInfoExceptionSpecificsRaw,
}

#[derive(FromBytes, AsBytes, FromZeroes, Copy, Clone)]
#[repr(C)]
struct DebugEventInfoProcessSpecificsRaw {
  title_id: U64,
  process_id: U64,
  process_name: [u8; 12],
  mmu_flags: U32,
  user_exception_context_addr: USIZE,
}

#[derive(FromBytes, AsBytes, FromZeroes, Copy, Clone)]
#[repr(C)]
struct DebugEventInfoThreadSpecificsRaw {
  thread_id: U64,
  tls_ptr: USIZE,
  entrypoint: USIZE,
}

#[derive(FromBytes, AsBytes, FromZeroes, Copy, Clone)]
#[repr(C)]
struct DebugEventInfoExitSpecificsRaw {
  r#type: U32,
}

#[derive(FromBytes, AsBytes, FromZeroes, Copy, Clone)]
#[repr(C)]
struct DebugEventInfoExceptionSpecificsRaw {
  exception_type: U32,
  fault_register: USIZE,
  argument_0: U32,
  argument_1: USIZE,
  argument_2: USIZE,
}

pub enum DebugEventInfo {
  Process(DebugEventProcess),
  Thread(DebugEventThread),
  ExitProcess(DebugEventExit),
  ExitThread(DebugEventExit),
  Exception(DebugEventException),
}

pub struct DebugEventProcess {
  pub flags: u32,
  pub thread_id: u64,
  pub title_id: u64,
  pub process_id: u64,
  pub process_name: [u8; 12],
  pub mmu_flags: u32,
  pub user_exception_context_address: usize,
}

pub struct DebugEventThread {
  pub flags: u32,
  pub thread_id: u64,
  pub thread_local_storage_pointer: usize,
  pub entrypoint: usize,
}

pub struct DebugEventExit {
  pub flags: u32,
  pub thread_id: u64,
  pub exit_kind: DebugEventExitKind,
}

pub enum DebugEventExitKind {
  PausedThread = 0,
  RunningThread = 1,
  ExitedProcess = 2,
  TerminatedProcess = 3,
}

impl DebugEventExitKind {
  fn opt_from(value: u32) -> Option<Self> {
    match value {
      0 => Some(DebugEventExitKind::PausedThread),
      1 => Some(DebugEventExitKind::RunningThread),
      2 => Some(DebugEventExitKind::ExitedProcess),
      3 => Some(DebugEventExitKind::TerminatedProcess),

      _ => None,
    }
  }
}

pub struct DebugEventException {
  pub flags: u32,
  pub thread_id: u64,
  pub fault_register: usize,
  pub exception_kind: DebugEventExceptionKind,
}

pub enum DebugEventExceptionKind {
  Trap { opcode: u32 },
  InstructionAbort,
  DataAbortMisc,
  ProgramCounterOrStackPointerAlignmentFault,
  DebuggerAttached,
  Breakpoint,
  Watchpoint,
  UserBreak(u32, usize, usize),
  DebuggerBreak,
  BadServiceCall { service_call_id: u32 },
  SystemError,
}

#[bitfield(u32)]
pub struct ContinueDebugFlags {
  ignore_exception: bool,
  dont_catch_exceptions: bool,
  resume: bool,
  ignore_other_threads_exceptions: bool,

  #[bits(28)]
  padding: u32,
}

#[derive(Clone, Copy)]
pub enum AddressSpaceType {
  AddressSpace32Bit = 0,
  AddressSpace64BitOld = 1,
  AddressSpace32BitNoReserved = 2,
  AddressSpace64Bit = 3,
}

#[derive(Clone, Copy)]
pub enum MemoryRegion {
  Application = 0,
  Applet = 1,
  SecureSystem = 2,
  NonSecureSystem = 3,
}

#[bitfield(u32)]
struct CreateProcessParameterFlagsRaw {
  is_64bit_instruction: bool,

  #[bits(4)]
  address_space_type: u8,

  enable_debug: bool,
  enable_aslr: bool,
  is_application: bool,
  use_secure_memory: bool,

  #[bits(4)]
  memory_region: u8,

  optimize_memory_allocation: bool,

  #[bits(18)]
  padding: u32,
}

#[derive(Clone, Copy)]
pub enum ProcessCategory {
  RegularTitle = 0,
  KernelBuiltin = 1,
}

pub struct CreateProcessParameter {
  name: [u8; 12],
  category: ProcessCategory,
  title_id: u64,
  code_addr: u64,
  code_num_pages: u32,
  is_64bit_instruction: bool,
  address_space_type: AddressSpaceType,
  enable_debug: bool,
  enable_aslr: bool,
  is_application: bool,
  use_secure_memory: bool,
  memory_region: MemoryRegion,
  optimize_memory_allocation: bool,
  resource_limit_handle: u32,
  system_resource_num_pages: u32,
}

impl CreateProcessParameter {
  pub fn new(
    name: heapless::String<12>,
    title_id: u64,
    code_addr: *mut c_void,
    code_num_pages: u32,
    memory_region: MemoryRegion,
    resource_limit: Handle<ResourceLimit>,
  ) -> CreateProcessParameter {
    let mut name_bytes: [u8; 12] = [0; 12];

    name_bytes[0..name.as_bytes().len()].copy_from_slice(name.as_bytes());

    CreateProcessParameter {
      name: name_bytes,
      category: ProcessCategory::RegularTitle,
      title_id,
      code_addr: code_addr as u64,
      code_num_pages,
      is_64bit_instruction: true,
      address_space_type: AddressSpaceType::AddressSpace64Bit,
      enable_aslr: true,
      enable_debug: false,
      is_application: false,
      memory_region: memory_region,
      optimize_memory_allocation: false,
      resource_limit_handle: resource_limit.as_bits(),
      system_resource_num_pages: 0,
      use_secure_memory: false,
    }
  }

  pub fn new_application(
    name: heapless::String<12>,
    title_id: u64,
    code_addr: *mut c_void,
    code_num_pages: u32,
    memory_region: MemoryRegion,
    resource_limit: Handle<ResourceLimit>,
  ) -> CreateProcessParameter {
    let mut name_bytes: [u8; 12] = [0; 12];

    name_bytes[0..name.as_bytes().len()].copy_from_slice(name.as_bytes());

    CreateProcessParameter {
      name: name_bytes,
      category: ProcessCategory::RegularTitle,
      title_id,
      code_addr: code_addr as u64,
      code_num_pages,
      is_64bit_instruction: true,
      address_space_type: AddressSpaceType::AddressSpace64Bit,
      enable_aslr: true,
      enable_debug: false,
      is_application: true,
      memory_region: memory_region,
      optimize_memory_allocation: false,
      resource_limit_handle: resource_limit.as_bits(),
      system_resource_num_pages: 0,
      use_secure_memory: false,
    }
  }

  pub fn with_category(self, category: ProcessCategory) -> Self {
    CreateProcessParameter { category, ..self }
  }

  pub fn with_address_space_type(self, address_space_type: AddressSpaceType) -> Self {
    CreateProcessParameter {
      address_space_type,
      ..self
    }
  }

  pub fn with_system_resource_page_count(self, system_resource_num_pages: u32) -> Self {
    CreateProcessParameter {
      system_resource_num_pages,
      ..self
    }
  }

  pub fn with_64bit_instructions(self) -> Self {
    CreateProcessParameter {
      is_64bit_instruction: true,
      ..self
    }
  }
  pub fn with_32bit_instructions(self) -> Self {
    CreateProcessParameter {
      is_64bit_instruction: false,
      ..self
    }
  }

  pub fn with_aslr(self) -> Self {
    CreateProcessParameter {
      enable_aslr: true,
      ..self
    }
  }
  pub fn without_aslr(self) -> Self {
    CreateProcessParameter {
      enable_aslr: false,
      ..self
    }
  }

  pub fn with_debug(self) -> Self {
    CreateProcessParameter {
      enable_debug: true,
      ..self
    }
  }
  pub fn without_debug(self) -> Self {
    CreateProcessParameter {
      enable_debug: false,
      ..self
    }
  }

  pub fn with_optimized_memory_allocations(self) -> Self {
    CreateProcessParameter {
      optimize_memory_allocation: true,
      ..self
    }
  }
  pub fn without_optimized_memory_allocations(self) -> Self {
    CreateProcessParameter {
      optimize_memory_allocation: false,
      ..self
    }
  }
}

#[derive(FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
struct CreateProcessParameterRaw {
  name: [u8; 12],
  category: U32,
  title_id: U64,
  code_addr: U64,
  code_num_pages: U32,
  flags: U32,
  resource_limit_handle: U32,
  system_resource_num_pages: U32,
}

pub enum ProcessState {
  Created,
  CreatedAttached,
  Started,
  Crashed,
  StartedAttached,
  Exiting,
  Exited,
  DebugSuspended,
}
