use alloc::string::String;
use alloc::vec::Vec;

/// Errors that can occur during filesystem operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    IoError,
}

/// Port (trait) that abstracts all filesystem interactions.
pub trait FileSystemPort {
    /// Read the full contents of the file at `path`.
    fn read(&self, path: &str) -> Result<Vec<u8>, FsError>;

    /// Write `data` to `path`, creating the file if it does not exist.
    fn write(&mut self, path: &str, data: &[u8]) -> Result<(), FsError>;

    /// Delete the file at `path`.
    fn delete(&mut self, path: &str) -> Result<(), FsError>;

    /// List the names of entries under directory `path`.
    fn list(&self, path: &str) -> Result<Vec<String>, FsError>;

    /// Return true if `path` exists in the filesystem.
    fn exists(&self, path: &str) -> bool;
}
