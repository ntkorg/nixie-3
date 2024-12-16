use zerocopy::FromZeroes;

use crate::result::result_code::ResultCode;
use core::arch::asm;
use super::{Debug, DebugEventException, DebugEventExceptionKind, DebugEventExit, DebugEventExitKind, DebugEventInfo, DebugEventInfoRaw, DebugEventProcess, DebugEventThread, Handle};

#[cfg(target_pointer_width = "64")]
fn get_debug_event_raw(handle: Handle<Debug>) -> Result<DebugEventInfoRaw, ResultCode> {
  let mut error_code: usize;
  let mut debug_event_info = DebugEventInfoRaw::new_zeroed();

  unsafe {
    asm!(
      "svc #0x63",
      
      in("w0") &mut debug_event_info as *mut DebugEventInfoRaw,
      in("w1") handle.as_bits(),
      lateout("x0") error_code,
      lateout("x1") _,
      lateout("x2") _,
      lateout("x3") _,
      lateout("x4") _,
      lateout("x5") _,
      lateout("x6") _,
      lateout("x7") _,
    );
  }

  if error_code == 0 {
    return Ok(debug_event_info);
  }

  Err(crate::result::result_code::ResultCode::from_bits(error_code as u32))
}

pub fn get_debug_event(handle: Handle<Debug>) -> Result<DebugEventInfo, ResultCode> {
  let raw = get_debug_event_raw(handle)?;

  unsafe {
    Ok(match raw.event_type.get() {
      0 => DebugEventInfo::Process(DebugEventProcess {
        flags: raw.flags.get(),
        thread_id: raw.thread_id.get(),
        title_id: raw.per_type_specifics.process.title_id.get(),
        process_id: raw.per_type_specifics.process.process_id.get(),
        process_name: raw.per_type_specifics.process.process_name,
        mmu_flags: raw.per_type_specifics.process.mmu_flags.get(),
        user_exception_context_address: raw.per_type_specifics.process.user_exception_context_addr.get() as usize,
      }),

      1 => {
        let thread_id_0 = raw.thread_id.get();
        let thread_id_1 = raw.per_type_specifics.thread.thread_id.get();

        if thread_id_0 != thread_id_1 {
          panic!("Expected thread id 0 and 1 to match. Kernel isn't implemented properly?");
        }

        DebugEventInfo::Thread(DebugEventThread {
          flags: raw.flags.get(),
          thread_id: thread_id_0,
          entrypoint: raw.per_type_specifics.thread.entrypoint.get() as usize,
          thread_local_storage_pointer: raw.per_type_specifics.thread.tls_ptr.get() as usize,
        })
      }

      2 => DebugEventInfo::ExitProcess(DebugEventExit {
        flags:  raw.flags.get(),
        thread_id: raw.thread_id.get(),
        exit_kind: DebugEventExitKind::opt_from(raw.per_type_specifics.exit.r#type.get()).expect("Invalid Exit Kind"),
      }),

      3 => DebugEventInfo::ExitProcess(DebugEventExit {
        flags: raw.flags.get(),
        thread_id: raw.thread_id.get(),
        exit_kind: DebugEventExitKind::opt_from(raw.per_type_specifics.exit.r#type.get()).expect("Invalid Exit Kind"),
      }),

      4 => DebugEventInfo::Exception(DebugEventException {
        flags: raw.flags.get(),
        thread_id: raw.thread_id.get(),
        fault_register: raw.per_type_specifics.exception.fault_register.get() as usize,
        exception_kind: match raw.per_type_specifics.exception.exception_type.get() {
          0 => DebugEventExceptionKind::Trap { opcode: raw.per_type_specifics.exception.argument_0.get() },
          1 => DebugEventExceptionKind::InstructionAbort,
          2 => DebugEventExceptionKind::DataAbortMisc,
          3 => DebugEventExceptionKind::ProgramCounterOrStackPointerAlignmentFault,
          4 => DebugEventExceptionKind::DebuggerAttached,
          5 => match raw.per_type_specifics.exception.argument_0.get() {
            0 => DebugEventExceptionKind::Breakpoint,
            _ => DebugEventExceptionKind::Watchpoint,
          },
          6 => DebugEventExceptionKind::UserBreak(
            raw.per_type_specifics.exception.argument_0.get(),
            raw.per_type_specifics.exception.argument_1.get() as usize,
            raw.per_type_specifics.exception.argument_2.get() as usize,
          ),
          7 => DebugEventExceptionKind::DebuggerBreak,
          8 => DebugEventExceptionKind::BadServiceCall { service_call_id: raw.per_type_specifics.exception.argument_0.get() },
          9 => DebugEventExceptionKind::SystemError,

          _ => panic!("Invalid Exception Kind"),
        }
      }),

      _ => unreachable!("Unknown event_type: {}", raw.event_type),
    })
  }
}