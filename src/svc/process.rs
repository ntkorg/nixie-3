use super::handle::Handle;

pub struct Process;

impl Process {
    pub fn current_process() -> Handle<Process> { Handle::new(0xFFFF8001) }
}
