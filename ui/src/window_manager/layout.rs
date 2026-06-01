use super::wm::{SnapZone, Window};

/// Compute the geometry (x, y, width, height) for a window snapped to a zone
/// within a screen of `screen_width` x `screen_height` pixels.
pub fn compute_snap_geometry(
    zone: &SnapZone,
    screen_width: u32,
    screen_height: u32,
) -> (u32, u32, u32, u32) {
    let hw = screen_width / 2;
    let hh = screen_height / 2;

    match zone {
        SnapZone::Left         => (0,  0,  hw,           screen_height),
        SnapZone::Right        => (hw, 0,  screen_width - hw, screen_height),
        SnapZone::TopLeft      => (0,  0,  hw, hh),
        SnapZone::TopRight     => (hw, 0,  screen_width - hw, hh),
        SnapZone::BottomLeft   => (0,  hh, hw, screen_height - hh),
        SnapZone::BottomRight  => (hw, hh, screen_width - hw, screen_height - hh),
        SnapZone::Fullscreen   => (0,  0,  screen_width, screen_height),
    }
}

/// Apply snap geometry to an existing window in place.
pub fn apply_snap(window: &mut Window, zone: &SnapZone, screen_width: u32, screen_height: u32) {
    let (x, y, w, h) = compute_snap_geometry(zone, screen_width, screen_height);
    window.x = x;
    window.y = y;
    window.width = w;
    window.height = h;
}
