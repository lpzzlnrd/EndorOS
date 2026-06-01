use egui::{Color32, FontId, FontFamily, ScrollArea, TextEdit, Ui};
use infrastructure::auth::local_auth::LocalAuthAdapter;
use infrastructure::crypto::aes_adapter::XorCryptoAdapter;
use infrastructure::fs::ramdisk::RamdiskAdapter;
use infrastructure::packages::pkg_manager::LocalPkgManager;
use infrastructure::process::scheduler::SchedulerAdapter;
use application::ports::auth::AuthPort;
use application::ports::encryption::EncryptionPort;
use application::ports::filesystem::FileSystemPort;
use application::ports::package_mgr::PackageManagerPort;
use application::ports::process_mgr::ProcessManagerPort;

use super::theme;

const DEFAULT_KEY: &[u8] = b"endoros-key-2024";

pub struct Terminal {
    pub history: Vec<(String, Color32)>,
    pub input: String,
    pub cmd_history: Vec<String>,   // command history for arrow-up navigation
    pub cmd_history_idx: Option<usize>,
    pub fs: RamdiskAdapter,
    pub auth: LocalAuthAdapter,
    pub proc_mgr: SchedulerAdapter,
    pub crypto: XorCryptoAdapter,
    pub pkg_mgr: LocalPkgManager,
    pub current_uid: Option<u32>,
    pub current_username: Option<String>,
    pub input_id: egui::Id,
}

impl Terminal {
    pub fn new(
        fs: RamdiskAdapter,
        auth: LocalAuthAdapter,
        proc_mgr: SchedulerAdapter,
        crypto: XorCryptoAdapter,
        pkg_mgr: LocalPkgManager,
    ) -> Self {
        let mut t = Self {
            history: Vec::new(),
            input: String::new(),
            cmd_history: Vec::new(),
            cmd_history_idx: None,
            fs,
            auth,
            proc_mgr,
            crypto,
            pkg_mgr,
            current_uid: None,
            current_username: None,
            input_id: egui::Id::new("terminal_input"),
        };
        t.push_output("EndorOS Shell v0.1.0", theme::TERM_GREEN);
        t.push_output("Type 'help' for available commands.", theme::TEXT_DIM);
        t.push_output("", theme::TEXT_DIM);
        t
    }

    pub fn push_output(&mut self, line: &str, color: Color32) {
        self.history.push((line.to_string(), color));
    }

    pub fn prompt_str(&self) -> String {
        let user = self.current_username.as_deref().unwrap_or("anonymous");
        format!("endoros@{}> ", user)
    }

    pub fn execute(&mut self, line: String) {
        let prompt = self.prompt_str();
        self.push_output(&format!("{}{}", prompt, line), theme::GREEN_GLOW);

        let trimmed = line.trim();
        if trimmed.is_empty() {
            return;
        }

        // Save to command history
        if !trimmed.is_empty() {
            self.cmd_history.push(trimmed.to_string());
        }
        self.cmd_history_idx = None;

        let parts: Vec<&str> = trimmed.splitn(4, ' ').collect();
        match parts[0] {
            "help" => self.cmd_help(),
            "exit" | "quit" => self.push_output("Use the window close button to exit.", theme::SAND),
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
            "clear" => self.history.clear(),
            _ => self.push_output(&format!("Unknown command: '{}'. Type 'help'.", parts[0]), Color32::from_rgb(200, 80, 80)),
        }
    }

    fn cmd_help(&mut self) {
        let cmds = [
            ("ps", "List running processes"),
            ("kill <pid>", "Terminate process"),
            ("run <name>", "Launch a process"),
            ("ls [path]", "List directory"),
            ("cat <path>", "Read file"),
            ("write <path> <content>", "Write file"),
            ("login <user> <pass>", "Authenticate"),
            ("logout", "Close session"),
            ("pkg install|remove|list|update [name]", "Package manager"),
            ("encrypt <text>", "XOR encrypt text"),
            ("whoami", "Current user"),
            ("clear", "Clear terminal"),
        ];
        self.push_output("Available commands:", theme::SAND);
        for (cmd, desc) in &cmds {
            self.push_output(&format!("  {:35} {}", cmd, desc), theme::TEXT_DIM);
        }
    }

    fn cmd_whoami(&mut self) {
        match &self.current_username.clone() {
            Some(u) => {
                let is_admin = self.current_uid.map(|uid| self.auth.is_admin(uid)).unwrap_or(false);
                let role = if is_admin { " [admin]" } else { " [user]" };
                self.push_output(&format!("{}{}", u, role), theme::TEXT);
            }
            None => self.push_output("Not logged in.", theme::SAND),
        }
    }

    fn cmd_ps(&mut self) {
        let procs = self.proc_mgr.list_processes();
        if procs.is_empty() {
            self.push_output("  No processes running.", theme::TEXT_DIM);
        } else {
            self.push_output("  PID   NAME", theme::SAND);
            for (pid, name) in &procs {
                self.push_output(&format!("  {:<5} {}", pid, name), theme::TEXT);
            }
        }
    }

    fn cmd_kill(&mut self, parts: &[&str]) {
        if parts.len() < 2 {
            self.push_output("Usage: kill <pid>", theme::SAND);
            return;
        }
        match parts[1].parse::<u32>() {
            Ok(pid) => match self.proc_mgr.kill(pid) {
                Ok(()) => self.push_output(&format!("Process {} terminated.", pid), theme::TERM_GREEN),
                Err(e) => self.push_output(&format!("Error: {:?}", e), Color32::from_rgb(200, 80, 80)),
            },
            Err(_) => self.push_output(&format!("Invalid PID: '{}'", parts[1]), Color32::from_rgb(200, 80, 80)),
        }
    }

    fn cmd_run(&mut self, parts: &[&str]) {
        if parts.len() < 2 {
            self.push_output("Usage: run <name>", theme::SAND);
            return;
        }
        match self.proc_mgr.spawn(parts[1], 5) {
            Ok(pid) => self.push_output(&format!("Launched '{}' with PID {}.", parts[1], pid), theme::TERM_GREEN),
            Err(e) => self.push_output(&format!("Error: {:?}", e), Color32::from_rgb(200, 80, 80)),
        }
    }

    fn cmd_ls(&mut self, parts: &[&str]) {
        let path = if parts.len() >= 2 { parts[1] } else { "/" };
        match self.fs.list(path) {
            Ok(entries) => {
                if entries.is_empty() {
                    self.push_output("  (empty)", theme::TEXT_MUTED);
                } else {
                    for e in entries {
                        self.push_output(&format!("  {}", e), theme::TEXT);
                    }
                }
            }
            Err(e) => self.push_output(&format!("Error: {:?}", e), Color32::from_rgb(200, 80, 80)),
        }
    }

    fn cmd_cat(&mut self, parts: &[&str]) {
        if parts.len() < 2 {
            self.push_output("Usage: cat <path>", theme::SAND);
            return;
        }
        match self.fs.read(parts[1]) {
            Ok(data) => {
                if data.is_empty() {
                    self.push_output("  (empty file)", theme::TEXT_MUTED);
                } else {
                    match std::str::from_utf8(&data) {
                        Ok(s) => {
                            for line in s.lines() {
                                self.push_output(line, theme::TERM_TEXT);
                            }
                        }
                        Err(_) => self.push_output(&format!("<binary data, {} bytes>", data.len()), theme::TEXT_MUTED),
                    }
                }
            }
            Err(e) => self.push_output(&format!("Error: {:?}", e), Color32::from_rgb(200, 80, 80)),
        }
    }

    fn cmd_write(&mut self, parts: &[&str]) {
        if parts.len() < 3 {
            self.push_output("Usage: write <path> <content>", theme::SAND);
            return;
        }
        match self.fs.write(parts[1], parts[2].as_bytes()) {
            Ok(()) => self.push_output(&format!("Written {} bytes to '{}'.", parts[2].len(), parts[1]), theme::TERM_GREEN),
            Err(e) => self.push_output(&format!("Error: {:?}", e), Color32::from_rgb(200, 80, 80)),
        }
    }

    fn cmd_login(&mut self, parts: &[&str]) {
        if parts.len() < 3 {
            self.push_output("Usage: login <username> <password>", theme::SAND);
            return;
        }
        match self.auth.authenticate(parts[1], parts[2]) {
            Ok(uid) => {
                self.current_uid = Some(uid);
                self.current_username = Some(parts[1].to_string());
                self.push_output(&format!("Welcome, {}! (uid={})", parts[1], uid), theme::TERM_GREEN);
            }
            Err(e) => self.push_output(&format!("Auth failed: {:?}", e), Color32::from_rgb(200, 80, 80)),
        }
    }

    fn cmd_logout(&mut self) {
        match self.current_uid {
            Some(uid) => {
                let name = self.current_username.clone().unwrap_or_default();
                match self.auth.logout(uid) {
                    Ok(()) => {
                        self.push_output(&format!("Logged out {}.", name), theme::TERM_GREEN);
                        self.current_uid = None;
                        self.current_username = None;
                    }
                    Err(e) => self.push_output(&format!("Error: {:?}", e), Color32::from_rgb(200, 80, 80)),
                }
            }
            None => self.push_output("No active session.", theme::SAND),
        }
    }

    fn cmd_pkg(&mut self, parts: &[&str]) {
        if parts.len() < 2 {
            self.push_output("Usage: pkg <install|remove|list|update> [name]", theme::SAND);
            return;
        }
        match parts[1] {
            "install" => {
                if parts.len() < 3 {
                    self.push_output("Usage: pkg install <name>", theme::SAND);
                    return;
                }
                match self.pkg_mgr.install(parts[2]) {
                    Ok(()) => self.push_output(&format!("Package '{}' installed.", parts[2]), theme::TERM_GREEN),
                    Err(e) => self.push_output(&format!("Error: {:?}", e), Color32::from_rgb(200, 80, 80)),
                }
            }
            "remove" => {
                if parts.len() < 3 {
                    self.push_output("Usage: pkg remove <name>", theme::SAND);
                    return;
                }
                match self.pkg_mgr.remove(parts[2]) {
                    Ok(()) => self.push_output(&format!("Package '{}' removed.", parts[2]), theme::TERM_GREEN),
                    Err(e) => self.push_output(&format!("Error: {:?}", e), Color32::from_rgb(200, 80, 80)),
                }
            }
            "list" => {
                let pkgs = self.pkg_mgr.list_installed();
                if pkgs.is_empty() {
                    self.push_output("  No packages installed.", theme::TEXT_DIM);
                } else {
                    self.push_output("  Installed packages:", theme::SAND);
                    for p in pkgs {
                        self.push_output(&format!("  â€¢ {}", p), theme::TEXT);
                    }
                }
            }
            "update" => match self.pkg_mgr.update_all() {
                Ok(()) => self.push_output("All packages updated.", theme::TERM_GREEN),
                Err(e) => self.push_output(&format!("Error: {:?}", e), Color32::from_rgb(200, 80, 80)),
            },
            other => self.push_output(&format!("Unknown pkg sub-command: '{}'", other), Color32::from_rgb(200, 80, 80)),
        }
    }

    fn cmd_encrypt(&mut self, parts: &[&str]) {
        if parts.len() < 2 {
            self.push_output("Usage: encrypt <text>", theme::SAND);
            return;
        }
        let text = parts[1..].join(" ");
        match self.crypto.encrypt(text.as_bytes(), DEFAULT_KEY) {
            Ok(cipher) => {
                let hex: String = cipher.iter().map(|b| format!("{:02x}", b)).collect();
                self.push_output(&format!("Cipher: {}", hex), theme::TERM_GREEN);
                if let Ok(plain) = self.crypto.decrypt(&cipher, DEFAULT_KEY) {
                    self.push_output(&format!("Verify: {}", String::from_utf8_lossy(&plain)), theme::TEXT_DIM);
                }
            }
            Err(e) => self.push_output(&format!("Error: {:?}", e), Color32::from_rgb(200, 80, 80)),
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        let available = ui.available_size();
        let output_height = available.y - 36.0;

        egui::Frame::none()
            .fill(theme::TERM_BG)
            .inner_margin(egui::Margin::same(8.0))
            .show(ui, |ui| {
                // Output scroll area
                ScrollArea::vertical()
                    .id_salt("terminal_scroll")
                    .auto_shrink([false, false])
                    .max_height(output_height - 16.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.style_mut().override_font_id = Some(FontId::new(13.0, FontFamily::Monospace));
                        for (line, color) in &self.history {
                            ui.colored_label(*color, line);
                        }
                    });

                ui.separator();

                // Handle arrow up/down for history BEFORE consuming keyboard events
                let arrow_up = ui.input(|i| i.key_pressed(egui::Key::ArrowUp));
                let arrow_down = ui.input(|i| i.key_pressed(egui::Key::ArrowDown));

                if arrow_up && !self.cmd_history.is_empty() {
                    let new_idx = match self.cmd_history_idx {
                        None => self.cmd_history.len() - 1,
                        Some(0) => 0,
                        Some(i) => i - 1,
                    };
                    self.cmd_history_idx = Some(new_idx);
                    self.input = self.cmd_history[new_idx].clone();
                }
                if arrow_down {
                    match self.cmd_history_idx {
                        None => {}
                        Some(i) if i + 1 >= self.cmd_history.len() => {
                            self.cmd_history_idx = None;
                            self.input.clear();
                        }
                        Some(i) => {
                            self.cmd_history_idx = Some(i + 1);
                            self.input = self.cmd_history[i + 1].clone();
                        }
                    }
                }

                // Input row
                ui.horizontal(|ui| {
                    ui.style_mut().override_font_id = Some(FontId::new(13.0, FontFamily::Monospace));
                    let prompt = self.prompt_str();
                    ui.colored_label(theme::GREEN_GLOW, &prompt);
                    let input_response = ui.add(
                        TextEdit::singleline(&mut self.input)
                            .id(self.input_id)
                            .desired_width(f32::INFINITY)
                            .frame(false)
                            .text_color(theme::TEXT)
                            .font(FontId::new(13.0, FontFamily::Monospace)),
                    );
                    if input_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let cmd = std::mem::take(&mut self.input);
                        self.execute(cmd);
                        input_response.request_focus();
                    }
                    if !input_response.has_focus() {
                        input_response.request_focus();
                    }
                });
            });
    }
}

