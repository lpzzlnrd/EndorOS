use std::collections::HashSet;
use application::ports::auth::{AuthError, AuthPort};

/// Hardcoded user database for demonstration purposes.
/// Tuple layout: (username, password, uid, is_admin).
pub struct LocalAuthAdapter {
    users: Vec<(String, String, u32, bool)>,
    /// Set of UIDs that are currently logged-in.
    active_sessions: HashSet<u32>,
}

impl LocalAuthAdapter {
    pub fn new() -> Self {
        Self {
            users: vec![
                ("admin".to_string(), "admin123".to_string(), 0, true),
                ("user".to_string(),  "user123".to_string(),  1, false),
                ("guest".to_string(), "guest".to_string(),    2, false),
            ],
            active_sessions: HashSet::new(),
        }
    }
}

impl Default for LocalAuthAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPort for LocalAuthAdapter {
    fn authenticate(&self, username: &str, password: &str) -> Result<u32, AuthError> {
        for (uname, upass, uid, _) in &self.users {
            if uname == username {
                if upass == password {
                    return Ok(*uid);
                } else {
                    return Err(AuthError::InvalidCredentials);
                }
            }
        }
        Err(AuthError::UserNotFound)
    }

    fn logout(&mut self, uid: u32) -> Result<(), AuthError> {
        if self.users.iter().any(|(_, _, u, _)| *u == uid) {
            self.active_sessions.remove(&uid);
            Ok(())
        } else {
            Err(AuthError::UserNotFound)
        }
    }

    fn is_admin(&self, uid: u32) -> bool {
        self.users
            .iter()
            .find(|(_, _, u, _)| *u == uid)
            .map(|(_, _, _, admin)| *admin)
            .unwrap_or(false)
    }
}
