use super::layout::apply_snap;

/// A single on-screen window with position and size.
#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Pre-defined snap zones that windows can be snapped into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapZone {
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Fullscreen,
}

/// Manages a collection of windows on a virtual screen.
pub struct WindowManager {
    windows: Vec<Window>,
    next_id: u32,
    screen_width: u32,
    screen_height: u32,
}

impl WindowManager {
    /// Create a new window manager for the given screen dimensions.
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            windows: Vec::new(),
            next_id: 1,
            screen_width,
            screen_height,
        }
    }

    /// Open a new window with the given `title` and default size (640x480 at origin).
    /// Returns the window ID.
    pub fn open_window(&mut self, title: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.windows.push(Window {
            id,
            title: title.to_string(),
            x: 0,
            y: 0,
            width: 640,
            height: 480,
        });
        id
    }

    /// Close the window with the given `id`. Returns true if a window was removed.
    pub fn close_window(&mut self, id: u32) -> bool {
        let before = self.windows.len();
        self.windows.retain(|w| w.id != id);
        self.windows.len() < before
    }

    /// Snap the window identified by `id` to the given `zone`.
    /// Does nothing if the window is not found.
    pub fn snap(&mut self, id: u32, zone: SnapZone) {
        if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
            apply_snap(win, &zone, self.screen_width, self.screen_height);
        }
    }

    /// Return a slice of all open windows.
    pub fn list_windows(&self) -> &[Window] {
        &self.windows
    }

    /// Print a textual representation of all windows to stdout.
    pub fn print_layout(&self) {
        if self.windows.is_empty() {
            println!("  (no open windows)");
            return;
        }
        for w in &self.windows {
            println!(
                "  [{}] \"{}\"  pos=({},{})  size={}x{}",
                w.id, w.title, w.x, w.y, w.width, w.height
            );
        }
    }
}
