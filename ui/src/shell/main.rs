use infrastructure::auth::local_auth::LocalAuthAdapter;
use infrastructure::crypto::aes_adapter::XorCryptoAdapter;
use infrastructure::fs::ramdisk::RamdiskAdapter;
use infrastructure::packages::pkg_manager::LocalPkgManager;
use infrastructure::process::scheduler::SchedulerAdapter;

use application::ports::filesystem::FileSystemPort;
use ui::shell::interpreter::Shell;
use ui::window_manager::wm::{SnapZone, WindowManager};

fn main() {
    // --- Boot banner -----------------------------------------------------------
    println!("┌─────────────────────────────────────────┐");
    println!("│          EndorOS  Kernel v0.1            │");
    println!("│   Hexagonal OS — Booting...              │");
    println!("└─────────────────────────────────────────┘");

    // --- Initialise infrastructure adapters -----------------------------------
    let mut fs = RamdiskAdapter::new();
    let auth = LocalAuthAdapter::new();
    let proc_mgr = SchedulerAdapter::new();
    let crypto = XorCryptoAdapter::new();
    let pkg_mgr = LocalPkgManager::new();

    // --- Mount initial VFS layout (simulated) ----------------------------------
    println!("[boot] Mounting ramdisk filesystem...");
    let dirs = ["/", "/bin", "/etc", "/home", "/tmp", "/var"];
    for dir in &dirs {
        if !fs.exists(dir) {
            fs.write(dir, b"").expect("failed to create directory sentinel");
        }
    }
    // Write a simple /etc/hostname
    fs.write("/etc/hostname", b"endoros-node-1").expect("hostname write failed");
    fs.write("/etc/motd", b"Welcome to EndorOS 0.1 - a conceptual hexagonal OS.").ok();
    println!("[boot] VFS mounted. Directories: {:?}", &dirs);

    // --- Initialise window manager (structural, no real display) ---------------
    let mut wm = WindowManager::new(1920, 1080);
    let term_id = wm.open_window("Terminal");
    wm.snap(term_id, SnapZone::Left);
    let editor_id = wm.open_window("Code Editor");
    wm.snap(editor_id, SnapZone::Right);
    println!("[boot] Window manager initialised.");
    println!("[boot] Open windows:");
    wm.print_layout();

    // --- Hand off to the Shell REPL -------------------------------------------
    println!("[boot] Starting EndorOS Shell...\n");
    let mut shell = Shell::new(fs, auth, proc_mgr, crypto, pkg_mgr);
    shell.run();

    println!("\n[shutdown] EndorOS halted. Goodbye.");
}
