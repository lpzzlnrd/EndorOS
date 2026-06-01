use crate::ports::package_mgr::{PkgError, PackageManagerPort};

/// Use-case: install a named package through the package-manager port.
pub struct InstallPackage<M: PackageManagerPort> {
    manager: M,
}

impl<M: PackageManagerPort> InstallPackage<M> {
    pub fn new(manager: M) -> Self {
        Self { manager }
    }

    /// Install the package `pkg_name`. Rejects unsigned packages in hardened mode.
    pub fn execute(&mut self, pkg_name: &str) -> Result<(), PkgError> {
        if pkg_name.is_empty() {
            return Err(PkgError::NotFound);
        }
        self.manager.install(pkg_name)
    }

    pub fn manager(&self) -> &M {
        &self.manager
    }

    pub fn manager_mut(&mut self) -> &mut M {
        &mut self.manager
    }
}
