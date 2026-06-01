use std::collections::HashMap;
use application::ports::filesystem::{FsError, FileSystemPort};

/// In-memory filesystem backed by a HashMap.
/// Keys are path strings; values are raw byte content.
/// Directories are represented as entries with empty content.
pub struct RamdiskAdapter {
    storage: HashMap<String, Vec<u8>>,
}

impl RamdiskAdapter {
    pub fn new() -> Self {
        let mut storage = HashMap::new();
        // Pre-create a minimal VFS layout.
        storage.insert("/".to_string(), Vec::new());
        Self { storage }
    }

    /// Return an iterator over paths that are direct children of `dir_path`.
    fn children_of(&self, dir_path: &str) -> Vec<&String> {
        let prefix = if dir_path == "/" {
            "/".to_string()
        } else {
            format!("{}/", dir_path)
        };
        self.storage.keys().filter(|k| {
            if dir_path == "/" {
                // Direct children of root: paths like "/foo" but not "/foo/bar"
                k.starts_with('/') && k.len() > 1 && !k[1..].contains('/')
            } else {
                k.starts_with(&prefix) && {
                    let rest = &k[prefix.len()..];
                    !rest.is_empty() && !rest.contains('/')
                }
            }
        }).collect()
    }
}

impl Default for RamdiskAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystemPort for RamdiskAdapter {
    fn read(&self, path: &str) -> Result<Vec<u8>, FsError> {
        self.storage
            .get(path)
            .cloned()
            .ok_or(FsError::NotFound)
    }

    fn write(&mut self, path: &str, data: &[u8]) -> Result<(), FsError> {
        // Ensure that the parent directory exists (simple check).
        if path != "/" {
            let parent = path.rfind('/').map(|i| if i == 0 { "/" } else { &path[..i] });
            if let Some(parent_path) = parent {
                if parent_path != "/" && !self.storage.contains_key(parent_path) {
                    // Auto-create missing parent as an empty directory.
                    self.storage.insert(parent_path.to_string(), Vec::new());
                }
            }
        }
        self.storage.insert(path.to_string(), data.to_vec());
        Ok(())
    }

    fn delete(&mut self, path: &str) -> Result<(), FsError> {
        if self.storage.remove(path).is_some() {
            Ok(())
        } else {
            Err(FsError::NotFound)
        }
    }

    fn list(&self, path: &str) -> Result<Vec<String>, FsError> {
        if !self.storage.contains_key(path) {
            return Err(FsError::NotFound);
        }
        let mut entries: Vec<String> = self.children_of(path).into_iter().cloned().collect();
        entries.sort();
        Ok(entries)
    }

    fn exists(&self, path: &str) -> bool {
        self.storage.contains_key(path)
    }
}
