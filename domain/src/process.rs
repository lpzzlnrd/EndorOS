/// Represents the execution state of a process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
}

/// A kernel process entity with a fixed-size name buffer to remain no_std compatible.
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    /// Process name stored as UTF-8 bytes in a fixed 64-byte array.
    pub name: [u8; 64],
    pub state: ProcessState,
    /// Scheduling priority, 0 = lowest, 255 = highest.
    pub priority: u8,
}

impl Process {
    /// Create a new process. `name_str` is truncated to 63 bytes if longer.
    pub fn new(pid: u32, name_str: &str, priority: u8) -> Self {
        let mut name = [0u8; 64];
        let bytes = name_str.as_bytes();
        let len = bytes.len().min(63);
        name[..len].copy_from_slice(&bytes[..len]);
        Self {
            pid,
            name,
            state: ProcessState::Running,
            priority,
        }
    }

    /// Returns the process name as a string slice (trimming trailing nulls).
    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        core::str::from_utf8(&self.name[..end]).unwrap_or("<invalid>")
    }
}
