use application::ports::package_mgr::{PkgError, PackageManagerPort};
use domain::package::{InstallState, Package};

/// Local package manager with a hardcoded catalogue and install tracking.
pub struct LocalPkgManager {
    installed: Vec<Package>,
    /// Names of packages available in the (simulated) remote catalogue.
    available: Vec<String>,
}

impl LocalPkgManager {
    pub fn new() -> Self {
        Self {
            installed: Vec::new(),
            available: vec![
                "bash".to_string(),
                "curl".to_string(),
                "vim".to_string(),
                "git".to_string(),
                "python3".to_string(),
                "gcc".to_string(),
                "make".to_string(),
                "openssl".to_string(),
                "net-tools".to_string(),
                "htop".to_string(),
            ],
        }
    }

    #[allow(dead_code)]
    fn find_installed_mut(&mut self, name: &str) -> Option<&mut Package> {
        self.installed.iter_mut().find(|p| p.name_str() == name)
    }

    fn is_installed(&self, name: &str) -> bool {
        self.installed
            .iter()
            .any(|p| p.name_str() == name && p.state == InstallState::Installed)
    }
}

impl Default for LocalPkgManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageManagerPort for LocalPkgManager {
    fn install(&mut self, pkg: &str) -> Result<(), PkgError> {
        if !self.available.iter().any(|a| a == pkg) {
            return Err(PkgError::NotFound);
        }
        if self.is_installed(pkg) {
            // Already installed — idempotent success.
            return Ok(());
        }
        let mut package = Package::new(pkg, "1.0.0", true);
        package.state = InstallState::Installing;
        // Simulate successful installation.
        package.state = InstallState::Installed;
        self.installed.push(package);
        Ok(())
    }

    fn remove(&mut self, pkg: &str) -> Result<(), PkgError> {
        let pos = self
            .installed
            .iter()
            .position(|p| p.name_str() == pkg && p.state == InstallState::Installed);
        match pos {
            Some(i) => {
                self.installed.remove(i);
                Ok(())
            }
            None => Err(PkgError::NotFound),
        }
    }

    fn list_installed(&self) -> Vec<String> {
        self.installed
            .iter()
            .filter(|p| p.state == InstallState::Installed)
            .map(|p| format!("{} ({})", p.name_str(), p.version_str()))
            .collect()
    }

    fn update_all(&mut self) -> Result<(), PkgError> {
        // Simulate bumping all installed packages to version 1.0.1.
        for pkg in self.installed.iter_mut() {
            if pkg.state == InstallState::Installed {
                let vbytes = b"1.0.1";
                pkg.version[..vbytes.len()].copy_from_slice(vbytes);
                pkg.version[vbytes.len()] = 0;
            }
        }
        Ok(())
    }
}
