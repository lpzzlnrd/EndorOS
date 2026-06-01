/// A directory entry representing either a file or a directory.
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// File or directory name, stored as UTF-8 bytes in a fixed 128-byte array.
    pub name: [u8; 128],
    /// Size in bytes; 0 for directories.
    pub size: usize,
    pub is_dir: bool,
}

impl FileEntry {
    /// Create a new file entry. `name_str` is truncated to 127 bytes if longer.
    pub fn new(name_str: &str, size: usize, is_dir: bool) -> Self {
        let mut name = [0u8; 128];
        let bytes = name_str.as_bytes();
        let len = bytes.len().min(127);
        name[..len].copy_from_slice(&bytes[..len]);
        Self { name, size, is_dir }
    }

    /// Returns the entry name as a string slice (trimming trailing nulls).
    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(128);
        core::str::from_utf8(&self.name[..end]).unwrap_or("<invalid>")
    }
}
