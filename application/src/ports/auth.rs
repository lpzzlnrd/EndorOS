/// Errors returned by authentication operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    InvalidCredentials,
    UserNotFound,
    SessionExpired,
}

/// Port (trait) that abstracts user authentication and session management.
pub trait AuthPort {
    /// Validate credentials and return the UID on success.
    fn authenticate(&self, username: &str, password: &str) -> Result<u32, AuthError>;

    /// Invalidate the session associated with `uid`.
    fn logout(&mut self, uid: u32) -> Result<(), AuthError>;

    /// Return true if the user identified by `uid` has the Admin role.
    fn is_admin(&self, uid: u32) -> bool;
}
