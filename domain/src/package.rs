/// Lifecycle state of a package installation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallState {
    Pending,
    Installing,
    Installed,
    Failed,
    RolledBack,
}

/// A software package with cryptographic signing metadata.
#[derive(Debug, Clone)]
pub struct Package {
    /// Package name stored as UTF-8 bytes in a fixed 64-byte array.
    pub name: [u8; 64],
    /// Semver-like version string in a fixed 16-byte array.
    pub version: [u8; 16],
    /// Whether the package carries a valid digital signature.
    pub signed: bool,
    pub state: InstallState,
}

impl Package {
    /// Create a new package entry. Names/versions are truncated to fit.
    pub fn new(name_str: &str, version_str: &str, signed: bool) -> Self {
        let mut name = [0u8; 64];
        let nb = name_str.as_bytes();
        let nlen = nb.len().min(63);
        name[..nlen].copy_from_slice(&nb[..nlen]);

        let mut version = [0u8; 16];
        let vb = version_str.as_bytes();
        let vlen = vb.len().min(15);
        version[..vlen].copy_from_slice(&vb[..vlen]);

        Self {
            name,
            version,
            signed,
            state: InstallState::Pending,
        }
    }

    /// Returns the package name as a string slice.
    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        core::str::from_utf8(&self.name[..end]).unwrap_or("<invalid>")
    }

    /// Returns the version string.
    pub fn version_str(&self) -> &str {
        let end = self.version.iter().position(|&b| b == 0).unwrap_or(16);
        core::str::from_utf8(&self.version[..end]).unwrap_or("<invalid>")
    }
}
