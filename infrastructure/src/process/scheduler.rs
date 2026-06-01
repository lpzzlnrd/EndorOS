use application::ports::process_mgr::{ProcessError, ProcessManagerPort};
use domain::process::{Process, ProcessState};

/// Simple round-robin-capable scheduler backed by a Vec of processes.
pub struct SchedulerAdapter {
    processes: Vec<Process>,
    next_pid: u32,
}

impl SchedulerAdapter {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            next_pid: 1,
        }
    }

    /// Return a reference to the process list (useful for inspection).
    pub fn processes(&self) -> &[Process] {
        &self.processes
    }
}

impl Default for SchedulerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessManagerPort for SchedulerAdapter {
    fn spawn(&mut self, name: &str, priority: u8) -> Result<u32, ProcessError> {
        if self.processes.len() >= 1024 {
            return Err(ProcessError::ResourceExhausted);
        }
        let pid = self.next_pid;
        self.next_pid += 1;
        self.processes.push(Process::new(pid, name, priority));
        Ok(pid)
    }

    fn kill(&mut self, pid: u32) -> Result<(), ProcessError> {
        if let Some(proc) = self.processes.iter_mut().find(|p| p.pid == pid) {
            proc.state = ProcessState::Zombie;
            // Remove zombie immediately for simplicity.
            self.processes.retain(|p| p.pid != pid);
            Ok(())
        } else {
            Err(ProcessError::NotFound)
        }
    }

    fn list_processes(&self) -> Vec<(u32, String)> {
        self.processes
            .iter()
            .map(|p| (p.pid, p.name_str().to_string()))
            .collect()
    }
}
