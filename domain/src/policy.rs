/// Security policy applied to a process or session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityPolicy {
    /// Whether the subject is allowed to execute binaries.
    pub allow_exec: bool,
    /// Whether the subject is allowed outbound network access.
    pub allow_network: bool,
    /// Whether all syscall-equivalent operations are logged for audit.
    pub audit_enabled: bool,
}

impl SecurityPolicy {
    /// Restrictive default: no exec, no network, audit on.
    pub fn default_hardened() -> Self {
        Self {
            allow_exec: false,
            allow_network: false,
            audit_enabled: true,
        }
    }

    /// Permissive policy used for the root/admin session during boot.
    pub fn permissive() -> Self {
        Self {
            allow_exec: true,
            allow_network: true,
            audit_enabled: false,
        }
    }

    /// Returns true when the policy allows execution of arbitrary binaries.
    pub fn can_exec(&self) -> bool {
        self.allow_exec
    }
}
