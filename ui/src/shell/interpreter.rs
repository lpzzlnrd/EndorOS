use std::io::{self, BufRead, Write};

use application::ports::auth::AuthPort;
use application::ports::encryption::EncryptionPort;
use application::ports::filesystem::FileSystemPort;
use application::ports::package_mgr::PackageManagerPort;
use application::ports::process_mgr::ProcessManagerPort;

/// The default encryption key used by the `encrypt` command (demo only).
const DEFAULT_KEY: &[u8] = b"endoros-key-2024";

/// Interactive shell REPL.
pub struct Shell<F, A, P, E, M>
where
    F: FileSystemPort,
    A: AuthPort,
    P: ProcessManagerPort,
    E: EncryptionPort,
    M: PackageManagerPort,
{
    fs: F,
    auth: A,
    proc_mgr: P,
    crypto: E,
    pkg_mgr: M,
    /// UID of the currently logged-in user (None = not logged in).
    current_uid: Option<u32>,
    current_username: Option<String>,
}

impl<F, A, P, E, M> Shell<F, A, P, E, M>
where
    F: FileSystemPort,
    A: AuthPort,
    P: ProcessManagerPort,
    E: EncryptionPort,
    M: PackageManagerPort,
{
    pub fn new(fs: F, auth: A, proc_mgr: P, crypto: E, pkg_mgr: M) -> Self {
        Self {
            fs,
            auth,
            proc_mgr,
            crypto,
            pkg_mgr,
            current_uid: None,
            current_username: None,
        }
    }

    /// Print the shell prompt.
    fn prompt(&self) {
        let user = self
            .current_username
            .as_deref()
            .unwrap_or("anonymous");
        print!("endoros@{}> ", user);
        io::stdout().flush().ok();
    }

    /// Parse and dispatch a single line of input.
    /// Returns `false` when the shell should exit.
    fn dispatch(&mut self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return true;
        }

        let parts: Vec<&str> = trimmed.splitn(4, ' ').collect();
        let cmd = parts[0];

        match cmd {
            "help" => self.cmd_help(),
            "exit" | "quit" => {
                println!("Goodbye.");
                return false;
            }
            "whoami" => self.cmd_whoami(),
            "ps" => self.cmd_ps(),
            "kill" => self.cmd_kill(&parts),
            "run" => self.cmd_run(&parts),
            "ls" => self.cmd_ls(&parts),
            "cat" => self.cmd_cat(&parts),
            "write" => self.cmd_write(&parts),
            "login" => self.cmd_login(&parts),
            "logout" => self.cmd_logout(),
            "pkg" => self.cmd_pkg(&parts),
            "encrypt" => self.cmd_encrypt(&parts),
            "wm" => self.cmd_wm(&parts),
            _ => println!("Unknown command: '{}'. Type 'help' for a list.", cmd),
        }

        true
    }

    fn cmd_help(&self) {
        println!("EndorOS Shell — available commands:");
        println!("  ps                        List running processes");
        println!("  kill <pid>                Terminate process by PID");
        println!("  run <name>                Launch process (priority 5)");
        println!("  ls <path>                 List directory contents");
        println!("  cat <path>                Print file contents");
        println!("  write <path> <content>    Write content to file");
        println!("  login <user> <pass>       Authenticate user");
        println!("  logout                    Close current session");
        println!("  pkg install <name>        Install a package");
        println!("  pkg remove <name>         Remove a package");
        println!("  pkg list                  List installed packages");
        println!("  pkg update                Update all packages");
        println!("  encrypt <text>            XOR-encrypt text (demo key)");
        println!("  whoami                    Show current user");
        println!("  wm open <title>           Open a window");
        println!("  wm close <id>             Close a window");
        println!("  wm snap <id> <zone>       Snap window (left/right/top-left/top-right/bottom-left/bottom-right/fullscreen)");
        println!("  wm list                   List open windows");
        println!("  help                      Show this help");
        println!("  exit / quit               Exit the shell");
    }

    fn cmd_whoami(&self) {
        match &self.current_username {
            Some(u) => {
                let admin = self
                    .current_uid
                    .map(|uid| self.auth.is_admin(uid))
                    .unwrap_or(false);
                println!("User: {}{}",
                    u,
                    if admin { " (admin)" } else { "" }
                );
            }
            None => println!("Not logged in."),
        }
    }

    fn cmd_ps(&self) {
        let procs = self.proc_mgr.list_processes();
        if procs.is_empty() {
            println!("  No processes running.");
        } else {
            println!("  PID   NAME");
            for (pid, name) in &procs {
                println!("  {:<5} {}", pid, name);
            }
        }
    }

    fn cmd_kill(&mut self, parts: &[&str]) {
        if parts.len() < 2 {
            println!("Usage: kill <pid>");
            return;
        }
        match parts[1].parse::<u32>() {
            Ok(pid) => match self.proc_mgr.kill(pid) {
                Ok(()) => println!("Process {} terminated.", pid),
                Err(e) => println!("Error: {:?}", e),
            },
            Err(_) => println!("Invalid PID: '{}'", parts[1]),
        }
    }

    fn cmd_run(&mut self, parts: &[&str]) {
        if parts.len() < 2 {
            println!("Usage: run <name>");
            return;
        }
        match self.proc_mgr.spawn(parts[1], 5) {
            Ok(pid) => println!("Launched '{}' with PID {}.", parts[1], pid),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    fn cmd_ls(&self, parts: &[&str]) {
        let path = if parts.len() >= 2 { parts[1] } else { "/" };
        match self.fs.list(path) {
            Ok(entries) => {
                if entries.is_empty() {
                    println!("  (empty)");
                } else {
                    for e in entries {
                        println!("  {}", e);
                    }
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    fn cmd_cat(&self, parts: &[&str]) {
        if parts.len() < 2 {
            println!("Usage: cat <path>");
            return;
        }
        match self.fs.read(parts[1]) {
            Ok(data) => {
                if data.is_empty() {
                    println!("  (empty file)");
                } else {
                    match std::str::from_utf8(&data) {
                        Ok(s) => println!("{}", s),
                        Err(_) => println!("  <binary data, {} bytes>", data.len()),
                    }
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    fn cmd_write(&mut self, parts: &[&str]) {
        // Usage: write <path> <content...>
        // parts is already splitn(4, ' ') so parts[2] is the rest of the line.
        if parts.len() < 3 {
            println!("Usage: write <path> <content>");
            return;
        }
        let path = parts[1];
        let content = parts[2];
        match self.fs.write(path, content.as_bytes()) {
            Ok(()) => println!("Written {} bytes to '{}'.", content.len(), path),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    fn cmd_login(&mut self, parts: &[&str]) {
        if parts.len() < 3 {
            println!("Usage: login <username> <password>");
            return;
        }
        match self.auth.authenticate(parts[1], parts[2]) {
            Ok(uid) => {
                self.current_uid = Some(uid);
                self.current_username = Some(parts[1].to_string());
                println!("Login successful. Welcome, {}! (uid={})", parts[1], uid);
            }
            Err(e) => println!("Authentication failed: {:?}", e),
        }
    }

    fn cmd_logout(&mut self) {
        match self.current_uid {
            Some(uid) => {
                match self.auth.logout(uid) {
                    Ok(()) => {
                        println!("Logged out {}.", self.current_username.as_deref().unwrap_or(""));
                        self.current_uid = None;
                        self.current_username = None;
                    }
                    Err(e) => println!("Logout error: {:?}", e),
                }
            }
            None => println!("No active session."),
        }
    }

    fn cmd_pkg(&mut self, parts: &[&str]) {
        if parts.len() < 2 {
            println!("Usage: pkg <install|remove|list|update> [name]");
            return;
        }
        match parts[1] {
            "install" => {
                if parts.len() < 3 {
                    println!("Usage: pkg install <name>");
                    return;
                }
                match self.pkg_mgr.install(parts[2]) {
                    Ok(()) => println!("Package '{}' installed successfully.", parts[2]),
                    Err(e) => println!("Install error: {:?}", e),
                }
            }
            "remove" => {
                if parts.len() < 3 {
                    println!("Usage: pkg remove <name>");
                    return;
                }
                match self.pkg_mgr.remove(parts[2]) {
                    Ok(()) => println!("Package '{}' removed.", parts[2]),
                    Err(e) => println!("Remove error: {:?}", e),
                }
            }
            "list" => {
                let pkgs = self.pkg_mgr.list_installed();
                if pkgs.is_empty() {
                    println!("  No packages installed.");
                } else {
                    for p in pkgs {
                        println!("  {}", p);
                    }
                }
            }
            "update" => match self.pkg_mgr.update_all() {
                Ok(()) => println!("All packages updated."),
                Err(e) => println!("Update error: {:?}", e),
            },
            other => println!("Unknown pkg sub-command: '{}'", other),
        }
    }

    fn cmd_encrypt(&self, parts: &[&str]) {
        if parts.len() < 2 {
            println!("Usage: encrypt <text>");
            return;
        }
        let text = parts[1..].join(" ");
        match self.crypto.encrypt(text.as_bytes(), DEFAULT_KEY) {
            Ok(cipher) => {
                let hex: String = cipher.iter().map(|b| format!("{:02x}", b)).collect();
                println!("Ciphertext (hex): {}", hex);
                // Also decrypt to demonstrate round-trip.
                if let Ok(plain) = self.crypto.decrypt(&cipher, DEFAULT_KEY) {
                    println!("Round-trip check: {}", String::from_utf8_lossy(&plain));
                }
            }
            Err(e) => println!("Encryption error: {:?}", e),
        }
    }

    fn cmd_wm(&self, parts: &[&str]) {
        // Window manager commands are handled by re-printing a note; the actual
        // WindowManager is accessible from main.rs where it is owned separately.
        println!("Window manager commands must be issued from within the WM context.");
        println!("Available sub-commands: open <title> | close <id> | snap <id> <zone> | list");
        println!("(Hint: the WM is accessible via the 'wm' module in main.rs)");
        let _ = parts; // suppress unused warning
    }

    /// Run the REPL until the user types 'exit' or 'quit'.
    pub fn run(&mut self) {
        println!("╔═══════════════════════════════════════╗");
        println!("║          EndorOS Shell v0.1.0         ║");
        println!("║   Type 'help' for available commands  ║");
        println!("╚═══════════════════════════════════════╝");

        let stdin = io::stdin();
        loop {
            self.prompt();
            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(0) => {
                    // EOF
                    println!();
                    break;
                }
                Ok(_) => {
                    if !self.dispatch(&line) {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    break;
                }
            }
        }
    }
}
