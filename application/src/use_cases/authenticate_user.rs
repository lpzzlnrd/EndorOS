use crate::ports::auth::{AuthError, AuthPort};

/// Use-case: verify user credentials and open a session.
pub struct AuthenticateUser<A: AuthPort> {
    auth: A,
}

impl<A: AuthPort> AuthenticateUser<A> {
    pub fn new(auth: A) -> Self {
        Self { auth }
    }

    /// Execute the use-case: validate `username`/`password` and return the UID.
    pub fn execute(&self, username: &str, password: &str) -> Result<u32, AuthError> {
        if username.is_empty() || password.is_empty() {
            return Err(AuthError::InvalidCredentials);
        }
        self.auth.authenticate(username, password)
    }

    pub fn auth(&self) -> &A {
        &self.auth
    }

    pub fn auth_mut(&mut self) -> &mut A {
        &mut self.auth
    }
}
