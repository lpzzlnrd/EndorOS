use alloc::string::String;
use alloc::vec::Vec;

/// Errors returned by process-management operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessError {
    NotFound,
    PermissionDenied,
    ResourceExhausted,
}

/// Port (trait) that abstracts process lifecycle management.
pub trait ProcessManagerPort {
    /// Spawn a new process with the given `name` and scheduling `priority`.
    /// Returns the assigned PID on success.
    fn spawn(&mut self, name: &str, priority: u8) -> Result<u32, ProcessError>;

    /// Terminate the process identified by `pid`.
    fn kill(&mut self, pid: u32) -> Result<(), ProcessError>;

    /// Return a snapshot of all running processes as (pid, name) pairs.
    fn list_processes(&self) -> Vec<(u32, String)>;
}
