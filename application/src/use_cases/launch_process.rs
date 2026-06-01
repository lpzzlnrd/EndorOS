use crate::ports::process_mgr::{ProcessError, ProcessManagerPort};

/// Use-case: spawn a new process through the process-manager port.
pub struct LaunchProcess<P: ProcessManagerPort> {
    manager: P,
}

impl<P: ProcessManagerPort> LaunchProcess<P> {
    pub fn new(manager: P) -> Self {
        Self { manager }
    }

    /// Execute the use-case: request the manager to spawn a process with the
    /// given `name` and `priority`. Returns the new PID on success.
    pub fn execute(&mut self, name: &str, priority: u8) -> Result<u32, ProcessError> {
        if name.is_empty() {
            return Err(ProcessError::PermissionDenied);
        }
        self.manager.spawn(name, priority)
    }

    /// Expose the inner manager (useful for chaining use-cases).
    pub fn manager(&self) -> &P {
        &self.manager
    }

    pub fn manager_mut(&mut self) -> &mut P {
        &mut self.manager
    }
}
