/// Role assigned to a user, controlling privilege level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
    Guest,
}

/// A system user with a fixed-size name buffer for no_std compatibility.
#[derive(Debug, Clone)]
pub struct User {
    pub uid: u32,
    /// Username stored as UTF-8 bytes in a fixed 32-byte array.
    pub name: [u8; 32],
    pub role: Role,
}

impl User {
    /// Create a new user. `name_str` is truncated to 31 bytes if longer.
    pub fn new(uid: u32, name_str: &str, role: Role) -> Self {
        let mut name = [0u8; 32];
        let bytes = name_str.as_bytes();
        let len = bytes.len().min(31);
        name[..len].copy_from_slice(&bytes[..len]);
        Self { uid, name, role }
    }

    /// Returns the username as a string slice (trimming trailing nulls).
    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        core::str::from_utf8(&self.name[..end]).unwrap_or("<invalid>")
    }
}

/// An active login session associated with a user.
#[derive(Debug, Clone)]
pub struct Session {
    pub user: User,
    pub active: bool,
    /// Session lifetime in seconds; 0 means no expiry.
    pub timeout_secs: u64,
}

impl Session {
    pub fn new(user: User, timeout_secs: u64) -> Self {
        Self {
            user,
            active: true,
            timeout_secs,
        }
    }

    pub fn invalidate(&mut self) {
        self.active = false;
    }
}
