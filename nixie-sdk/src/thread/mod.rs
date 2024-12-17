use local_region::{get_local_region, get_mut_local_region, LocalRegion};
use standalone::{StandaloneData, StandaloneThreadData};
use zerocopy::FromBytes;

use crate::svc::Handle;

pub mod local_region;
pub mod standalone;

pub struct Thread {}

impl !Send for Thread {}

impl Thread {
  pub (crate) unsafe fn initialize_standalone_main_thread_from_handle(handle: Handle<crate::svc::Thread>) {
    assert!(!cfg!(feature = "nnsdk_sidecar"));

    unsafe {
      let local_region = get_mut_local_region();
      let local_region = &mut *local_region;
      let data = StandaloneData::mut_from(&mut local_region.content).unwrap();
      
      data.set_data(StandaloneThreadData {
        handle,
      });
    }
  }

  pub fn current() -> &'static StandaloneThreadData {
    if cfg!(feature = "nnsdk_sidecar") {
      todo!()
    } else {
      StandaloneData::ref_from(&get_local_region().content).unwrap().get_data()
    }
  }
}
