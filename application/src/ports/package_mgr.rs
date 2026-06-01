use alloc::string::String;
use alloc::vec::Vec;

/// Errors returned by package-management operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PkgError {
    NotFound,
    SignatureInvalid,
    DependencyConflict,
    RollbackFailed,
}

/// Port (trait) that abstracts software package management.
pub trait PackageManagerPort {
    /// Install the package identified by `pkg` name.
    fn install(&mut self, pkg: &str) -> Result<(), PkgError>;

    /// Remove a previously installed package.
    fn remove(&mut self, pkg: &str) -> Result<(), PkgError>;

    /// Return the names of all currently installed packages.
    fn list_installed(&self) -> Vec<String>;

    /// Update all installed packages to the latest available version.
    fn update_all(&mut self) -> Result<(), PkgError>;
}
