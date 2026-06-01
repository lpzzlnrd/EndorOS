use alloc::vec::Vec;
use crate::ports::filesystem::{FsError, FileSystemPort};

/// Use-case: mount (initialise) a filesystem by writing sentinel entries.
pub struct MountFilesystem<F: FileSystemPort> {
    fs: F,
}

impl<F: FileSystemPort> MountFilesystem<F> {
    pub fn new(fs: F) -> Self {
        Self { fs }
    }

    /// Initialise the filesystem with root-level sentinel directories.
    /// Returns the list of paths that were created.
    pub fn execute(&mut self) -> Result<Vec<&'static str>, FsError> {
        let dirs: &[&str] = &["/", "/bin", "/etc", "/home", "/tmp", "/var"];
        for &dir in dirs {
            if !self.fs.exists(dir) {
                // Write an empty marker so the directory is visible via list().
                self.fs.write(dir, b"")?;
            }
        }
        Ok(dirs.to_vec())
    }

    pub fn fs(&self) -> &F {
        &self.fs
    }

    pub fn fs_mut(&mut self) -> &mut F {
        &mut self.fs
    }
}
