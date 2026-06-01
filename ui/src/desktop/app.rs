use egui::{CentralPanel, Color32, Context, Frame, Margin, Painter, Pos2, Rect, Rounding, SidePanel, Stroke, TopBottomPanel, Vec2};
use infrastructure::auth::local_auth::LocalAuthAdapter;
use infrastructure::crypto::aes_adapter::XorCryptoAdapter;
use infrastructure::fs::ramdisk::RamdiskAdapter;
use infrastructure::packages::pkg_manager::LocalPkgManager;
use infrastructure::process::scheduler::SchedulerAdapter;
use application::ports::filesystem::FileSystemPort;
use application::ports::package_mgr::PackageManagerPort;
use application::ports::process_mgr::ProcessManagerPort;

use super::taskbar::{AppIcon, Taskbar};
use super::terminal::Terminal;
use super::file_explorer::FileExplorer;
use super::process_monitor::ProcessMonitor;
use super::theme;
use super::widgets;

#[derive(PartialEq, Clone)]
enum ActivePanel { Dashboard, Terminal, Files, Processes, Packages, Settings }

pub struct EndorApp {
    taskbar: Taskbar,
    terminal: Terminal,
    file_explorer: FileExplorer,
    process_monitor: ProcessMonitor,
    active: ActivePanel,
    pkg_mgr: LocalPkgManager,
    pkg_status: String,
    pkg_input: String,
    settings_tab: usize,
    boot_time: std::time::Instant,
}

impl EndorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        super::theme::apply_no_font(&cc.egui_ctx);

        let mut fs = RamdiskAdapter::new();
        let auth = LocalAuthAdapter::new();
        let mut proc_mgr = SchedulerAdapter::new();
        let crypto = XorCryptoAdapter::new();
        let pkg_mgr_terminal = LocalPkgManager::new();
        let pkg_mgr_panel = LocalPkgManager::new();

        // VFS
        for dir in &["/", "/bin", "/etc", "/home", "/home/admin", "/home/admin/docs",
                     "/tmp", "/var", "/var/log", "/usr", "/usr/lib", "/usr/bin"] {
            let _ = fs.write(dir, b"");
        }
        let _ = fs.write("/etc/hostname",   b"endoros-node-1");
        let _ = fs.write("/etc/motd",       b"Welcome to EndorOS 0.1 - the forest moon OS.");
        let _ = fs.write("/etc/os-release", b"NAME=EndorOS\nVERSION=0.1.0\nARCH=x86_64\nAUTHOR=Leonardo Correa\nINSTITUTION=UJAP");
        let _ = fs.write("/etc/hosts",      b"127.0.0.1   localhost\n127.0.1.1   endoros-node-1\n::1         localhost");
        let _ = fs.write("/etc/fstab",      b"# <device>   <mountpoint>   <type>\nramdisk  /    ramfs  defaults\ntmpfs    /tmp tmpfs  nosuid");
        let _ = fs.write("/etc/security.conf", b"[policy]\ndefault=hardened\naudit=enabled\nprocess_isolation=true\nsign_packages=true\nsession_timeout=3600");

        let _ = fs.write("/home/admin/readme.txt",
            b"# Bienvenido a EndorOS\n\nEste es tu directorio home.\n\nComandos utiles:\n  login admin admin123   - iniciar sesion\n  ls /                   - listar raiz\n  ps                     - ver procesos\n  pkg install rustc      - instalar paquete\n  help                   - ver todos los comandos");
        let _ = fs.write("/home/admin/notas.txt",
            b"Notas de diseno EndorOS:\n\n1. Arquitectura hexagonal (Ports & Adapters)\n2. Kernel no_std con bump allocator 2MiB\n3. VFS en ramdisk para prototipo\n4. Auth local hardcoded para demo\n5. Cifrado XOR - reemplazar con AES-256-XTS en produccion\n\nTODO:\n- [ ] Driver real de disco\n- [ ] Networking stack\n- [ ] Proceso de arranque completo");
        let _ = fs.write("/home/admin/docs/arquitectura.md",
            b"# Arquitectura EndorOS\n\n## Capas\n\nUI (egui)         <- esta aplicacion\n  |\nApplication       <- casos de uso + ports (traits)\n  |\nDomain            <- entidades puras (no_std)\n  |\nInfrastructure    <- adaptadores concretos (std)\n\n## Ports implementados\n- FileSystemPort  -> RamdiskAdapter\n- AuthPort        -> LocalAuthAdapter\n- ProcessMgrPort  -> SchedulerAdapter\n- EncryptionPort  -> XorCryptoAdapter\n- PackageMgrPort  -> LocalPkgManager");
        let _ = fs.write("/home/admin/docs/seguridad.md",
            b"# Modelo de Seguridad EndorOS\n\nBasado en el principio de minimo privilegio.\n\n## Roles\n- admin (uid=0): acceso total\n- user  (uid=1): acceso estandar\n- guest (uid=2): solo lectura\n\n## Politicas\n- Aislamiento de procesos activado por defecto\n- Paquetes requieren firma criptografica\n- Audit log habilitado\n- Session timeout: 3600s");

        let _ = fs.write("/tmp/boot.log",
            b"[0.001] Kernel init - EndorOS 0.1.0\n[0.012] Bump allocator OK (2 MiB)\n[0.015] GDT/IDT loaded\n[0.020] CPU: SSE4.2 AVX2\n[0.031] VFS mounted - RamdiskFS\n[0.035] Auth adapter init\n[0.038] Crypto adapter init\n[0.041] Package manager ready\n[0.045] Scheduler started\n[0.060] Desktop loaded\n[0.065] EndorOS ready");
        let _ = fs.write("/var/log/system.log",
            b"[INFO]  EndorOS desktop started\n[INFO]  User session initialized\n[INFO]  VFS mounted at /\n[INFO]  5 system processes spawned\n[AUDIT] Login attempt: none\n[INFO]  GUI: egui 0.29");
        let _ = fs.write("/var/log/auth.log",
            b"[AUTH] System boot - auth daemon ready\n[AUTH] Users loaded: admin, user, guest\n[AUTH] Session store initialized");

        let _ = fs.write("/usr/lib/libendor.so",   b"ELF64 shared library - EndorOS runtime");
        let _ = fs.write("/usr/bin/endoros-sh",    b"#!/usr/bin/endoros-sh\n# EndorOS Shell v0.1");
        let _ = fs.write("/bin/ls",   b"#!/usr/bin/endoros-sh\n# list directory contents");
        let _ = fs.write("/bin/cat",  b"#!/usr/bin/endoros-sh\n# concatenate and print files");
        let _ = fs.write("/bin/ps",   b"#!/usr/bin/endoros-sh\n# report process status");
        let _ = fs.write("/bin/kill", b"#!/usr/bin/endoros-sh\n# send signal to process");

        // Demo processes
        let _ = proc_mgr.spawn("init",             255);
        let _ = proc_mgr.spawn("kernel-sched",     200);
        let _ = proc_mgr.spawn("vfs-daemon",       150);
        let _ = proc_mgr.spawn("auth-daemon",      140);
        let _ = proc_mgr.spawn("endoros-desktop",  100);

        let terminal = Terminal::new(fs, auth, proc_mgr, crypto, pkg_mgr_terminal);

        Self {
            taskbar: Taskbar::new(),
            terminal,
            file_explorer: FileExplorer::new(),
            process_monitor: ProcessMonitor::new(),
            active: ActivePanel::Dashboard,
            pkg_mgr: pkg_mgr_panel,
            pkg_status: String::new(),
            pkg_input: String::new(),
            settings_tab: 0,
            boot_time: std::time::Instant::now(),
        }
    }
}

impl eframe::App for EndorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(500));

        self.taskbar.update_clock();
        self.taskbar.username = self.terminal.current_username
            .clone()
            .unwrap_or("anonymous".to_string());

        self.taskbar.open_apps.clear();
        let open_icon = match &self.active {
            ActivePanel::Terminal  => Some(AppIcon::Terminal),
            ActivePanel::Files     => Some(AppIcon::Files),
            ActivePanel::Processes => Some(AppIcon::Processes),
            ActivePanel::Packages  => Some(AppIcon::Packages),
            ActivePanel::Settings  => Some(AppIcon::Settings),
            ActivePanel::Dashboard => None,
        };
        if let Some(icon) = open_icon { self.taskbar.open_apps.push(icon); }

        // Taskbar
        TopBottomPanel::bottom("taskbar")
            .exact_height(44.0)
            .frame(Frame::none().fill(theme::TASKBAR_BG))
            .show(ctx, |ui| {
                if let Some(app) = self.taskbar.render(ui) {
                    let target = match app {
                        AppIcon::Terminal  => ActivePanel::Terminal,
                        AppIcon::Files     => ActivePanel::Files,
                        AppIcon::Processes => ActivePanel::Processes,
                        AppIcon::Packages  => ActivePanel::Packages,
                        AppIcon::Settings  => ActivePanel::Settings,
                    };
                    self.active = if self.active == target { ActivePanel::Dashboard } else { target };
                }
            });

        // Sidebar
        SidePanel::left("sidebar")
            .exact_width(56.0)
            .resizable(false)
            .frame(Frame::none().fill(theme::SIDEBAR_BG))
            .show(ctx, |ui| {
                ui.add_space(10.0);

                let (logo_rect, _) = ui.allocate_exact_size(Vec2::new(56.0, 40.0), egui::Sense::hover());
                ui.painter().circle_filled(logo_rect.center(), 16.0, theme::with_alpha(theme::GREEN_DIM, 60));
                ui.painter().circle_stroke(logo_rect.center(), 16.0, Stroke::new(1.0, theme::GREEN_DIM));
                ui.painter().text(
                    logo_rect.center(), egui::Align2::CENTER_CENTER,
                    "E", egui::FontId::proportional(18.0), theme::GREEN_GLOW,
                );

                ui.add_space(8.0);
                let sep = ui.min_rect();
                ui.painter().line_segment(
                    [Pos2::new(sep.min.x + 8.0, sep.max.y), Pos2::new(sep.max.x - 8.0, sep.max.y)],
                    Stroke::new(1.0, theme::BORDER),
                );
                ui.add_space(8.0);

                let items: &[(&str, &str, ActivePanel)] = &[
                    (">_", "Terminal",      ActivePanel::Terminal),
                    ("[]", "File Explorer", ActivePanel::Files),
                    ("##", "Processes",     ActivePanel::Processes),
                    ("<>", "Packages",      ActivePanel::Packages),
                    ("==", "Settings",      ActivePanel::Settings),
                ];

                for (glyph, tooltip, panel) in items {
                    let active = self.active == *panel;
                    if sidebar_btn(ui, glyph, tooltip, active).clicked() {
                        self.active = if active { ActivePanel::Dashboard } else { panel.clone() };
                    }
                    ui.add_space(2.0);
                }
            });

        // Main content
        CentralPanel::default()
            .frame(Frame::none().fill(theme::BG_BASE))
            .show(ctx, |ui| {
                match &self.active {
                    ActivePanel::Dashboard => {
                        render_dashboard(ui, &self.terminal, self.boot_time);
                    }
                    ActivePanel::Terminal => {
                        render_panel(ui, "Terminal", theme::GREEN, |ui| {
                            self.terminal.render(ui);
                        });
                    }
                    ActivePanel::Files => {
                        let fs = &mut self.terminal.fs;
                        render_panel(ui, "File Explorer", theme::SAND, |ui| {
                            self.file_explorer.render(ui, fs);
                        });
                    }
                    ActivePanel::Processes => {
                        let pm = &mut self.terminal.proc_mgr;
                        render_panel(ui, "Process Monitor", theme::GREEN_GLOW, |ui| {
                            self.process_monitor.render(ui, pm);
                        });
                    }
                    ActivePanel::Packages => {
                        let pm = &mut self.pkg_mgr;
                        let pi = &mut self.pkg_input;
                        let ps = &mut self.pkg_status;
                        render_panel(ui, "Package Manager", theme::STONE, |ui| {
                            render_packages(ui, pm, pi, ps);
                        });
                    }
                    ActivePanel::Settings => {
                        let st = &mut self.settings_tab;
                        render_panel(ui, "Settings", theme::TEXT_DIM, |ui| {
                            render_settings(ui, st);
                        });
                    }
                }
            });
    }
}

fn sidebar_btn(ui: &mut egui::Ui, glyph: &str, tooltip: &str, active: bool) -> egui::Response {
    let size = Vec2::new(48.0, 40.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let painter = ui.painter();

    if active {
        painter.rect_filled(rect, Rounding::same(6.0), theme::BG_ACTIVE);
        painter.rect_filled(
            Rect::from_min_size(rect.min, Vec2::new(2.5, rect.height())),
            Rounding { nw: 6.0, sw: 6.0, ne: 0.0, se: 0.0 },
            theme::GREEN,
        );
    } else if response.hovered() {
        painter.rect_filled(rect, Rounding::same(6.0), theme::BG_HOVER);
    }

    let color = if active { theme::GREEN_GLOW } else if response.hovered() { theme::TEXT } else { theme::TEXT_MUTED };
    painter.text(rect.center(), egui::Align2::CENTER_CENTER, glyph, egui::FontId::monospace(14.0), color);
    response.on_hover_text(tooltip)
}

fn render_panel(ui: &mut egui::Ui, title: &str, accent: Color32, content: impl FnOnce(&mut egui::Ui)) {
    Frame::none().fill(theme::BG_PANEL).show(ui, |ui| {
        widgets::title_bar(ui, title, accent);
        Frame::none().fill(theme::BG_PANEL).inner_margin(Margin::same(14.0)).show(ui, content);
    });
}

fn render_dashboard(ui: &mut egui::Ui, terminal: &Terminal, boot_time: std::time::Instant) {
    let uptime_secs = boot_time.elapsed().as_secs();
    let uptime = format!("{}:{:02}", uptime_secs / 60, uptime_secs % 60);
    let proc_count = terminal.proc_mgr.list_processes().len().to_string();
    let user = terminal.current_username.as_deref().unwrap_or("anonymous").to_string();

    let full = ui.max_rect();
    paint_nebula(ui.painter(), full);

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(28.0);

        // Header
        ui.vertical_centered(|ui| {
            let (ring_rect, _) = ui.allocate_exact_size(Vec2::splat(72.0), egui::Sense::hover());
            let p = ui.painter();
            p.circle_filled(ring_rect.center(), 36.0, theme::with_alpha(theme::GREEN_DIM, 40));
            p.circle_stroke(ring_rect.center(), 36.0, Stroke::new(1.5, theme::GREEN_DIM));
            p.circle_stroke(ring_rect.center(), 30.0, Stroke::new(0.5, theme::with_alpha(theme::GREEN_DIM, 60)));
            p.text(ring_rect.center(), egui::Align2::CENTER_CENTER, "E", egui::FontId::proportional(34.0), theme::GREEN_GLOW);

            ui.add_space(10.0);
            ui.label(egui::RichText::new("EndorOS").color(theme::GREEN_GLOW).size(30.0).strong());
            ui.label(egui::RichText::new("Hexagonal OS  v0.1.0  x86_64 no_std").color(theme::TEXT_MUTED).size(11.5));
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 200.0) / 2.0);
                widgets::pill_badge(ui, "Running", theme::GREEN);
                ui.add_space(6.0);
                widgets::pill_badge(ui, "Hardened", theme::GREEN_GLOW);
                ui.add_space(6.0);
                widgets::pill_badge(ui, "Hexagonal", theme::SAND);
            });
        });

        ui.add_space(22.0);

        // Stat cards
        let total_w = ui.available_width() - 28.0;
        let card_w = (total_w - 36.0) / 4.0;
        ui.horizontal(|ui| {
            ui.add_space(14.0);
            for (label, value, accent) in &[
                ("Uptime",    uptime.as_str(),     theme::GREEN),
                ("Processes", proc_count.as_str(), theme::GREEN_GLOW),
                ("Session",   user.as_str(),        theme::SAND),
                ("Kernel",    "0.1.0",             theme::STONE),
            ] {
                ui.allocate_ui(Vec2::new(card_w, 72.0), |ui| {
                    widgets::stat_card(ui, label, value, *accent);
                });
                ui.add_space(9.0);
            }
        });

        ui.add_space(22.0);

        // Two columns
        let avail = ui.available_width() - 28.0;
        let col = (avail - 14.0) / 2.0;

        ui.horizontal_top(|ui| {
            ui.add_space(14.0);

            // Left column: system info
            ui.allocate_ui(Vec2::new(col, 320.0), |ui| {
                fancy_card(ui, "Sistema", theme::GREEN, |ui| {
                    widgets::info_row(ui, "Hostname",   "endoros-node-1");
                    widgets::info_row(ui, "Arch",       "x86_64 (no_std)");
                    widgets::info_row(ui, "Kernel",     "EndorOS 0.1.0");
                    widgets::info_row(ui, "Filesystem", "RamdiskFS");
                    widgets::info_row(ui, "Scheduler",  "Round-robin");
                    widgets::info_row(ui, "Auth",       "LocalAuthAdapter");
                    widgets::info_row(ui, "Crypto",     "XorCryptoAdapter");
                    ui.add_space(6.0);
                    widgets::labeled_separator(ui, "Autor");
                    widgets::info_row(ui, "Nombre",     "Leonardo Correa");
                    widgets::info_row(ui, "C.I.",       "30889380");
                    widgets::info_row(ui, "UJAP",       "Escuela de Computacion");
                });
            });

            ui.add_space(14.0);

            // Right column: processes + boot log
            ui.allocate_ui(Vec2::new(col, 320.0), |ui| {
                fancy_card(ui, "Procesos activos", theme::GREEN_GLOW, |ui| {
                    let procs = terminal.proc_mgr.list_processes();
                    egui::ScrollArea::vertical().id_salt("dash_proc").max_height(110.0).show(ui, |ui| {
                        egui::Grid::new("dash_proc_grid").num_columns(2).spacing([12.0, 3.0]).show(ui, |ui| {
                            for (pid, name) in &procs {
                                ui.label(egui::RichText::new(format!("{:>4}", pid)).color(theme::GREEN_GLOW).monospace().size(11.5));
                                ui.label(egui::RichText::new(name.clone()).color(theme::TEXT).monospace().size(11.5));
                                ui.end_row();
                            }
                        });
                    });
                });

                ui.add_space(10.0);

                fancy_card(ui, "Boot log", theme::STONE, |ui| {
                    Frame::none()
                        .fill(theme::TERM_BG)
                        .rounding(Rounding::same(5.0))
                        .inner_margin(Margin::same(8.0))
                        .show(ui, |ui| {
                            for line in &[
                                "[0.001] Kernel init OK",
                                "[0.012] Bump allocator ready (2 MiB)",
                                "[0.031] VFS mounted - RamdiskFS",
                                "[0.038] Auth adapter loaded",
                                "[0.041] Crypto adapter ready",
                                "[0.045] Scheduler started",
                                "[0.065] Desktop environment loaded",
                            ] {
                                ui.label(egui::RichText::new(*line).color(theme::TERM_GREEN).monospace().size(10.5));
                            }
                        });
                });
            });
        });

        ui.add_space(20.0);
    });
}

fn fancy_card(ui: &mut egui::Ui, title: &str, accent: Color32, content: impl FnOnce(&mut egui::Ui)) {
    Frame::none()
        .fill(theme::BG_CARD)
        .stroke(Stroke::new(1.0, theme::BORDER_LIT))
        .rounding(Rounding::same(8.0))
        .show(ui, |ui| {
            Frame::none()
                .fill(theme::with_alpha(accent, 18))
                .rounding(Rounding { nw: 8.0, ne: 8.0, sw: 0.0, se: 0.0 })
                .inner_margin(Margin::symmetric(12.0, 7.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let (r, _) = ui.allocate_exact_size(Vec2::splat(8.0), egui::Sense::hover());
                        ui.painter().circle_filled(r.center(), 4.0, accent);
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new(title).color(accent).size(12.0).strong());
                    });
                });
            let r = ui.min_rect();
            ui.painter().line_segment(
                [Pos2::new(r.min.x, r.max.y), Pos2::new(r.max.x, r.max.y)],
                Stroke::new(1.0, theme::with_alpha(accent, 60)),
            );
            Frame::none().inner_margin(Margin::same(12.0)).show(ui, content);
        });
}

fn paint_nebula(painter: &Painter, rect: Rect) {
    painter.rect_filled(rect, Rounding::ZERO, theme::BG_BASE);
    let c1 = Pos2::new(rect.min.x + rect.width() * 0.15, rect.min.y + rect.height() * 0.35);
    for (r, a) in &[(220.0_f32, 8_u8), (140.0, 12), (70.0, 16)] {
        painter.circle_filled(c1, *r, theme::with_alpha(theme::GREEN_DIM, *a));
    }
    let c2 = Pos2::new(rect.min.x + rect.width() * 0.82, rect.min.y + rect.height() * 0.25);
    for (r, a) in &[(180.0_f32, 6_u8), (100.0, 10), (50.0, 14)] {
        painter.circle_filled(c2, *r, theme::with_alpha(theme::SAND, *a));
    }
}

fn render_packages(ui: &mut egui::Ui, pkg_mgr: &mut LocalPkgManager, input: &mut String, status: &mut String) {
    fancy_card(ui, "Install / Remove", theme::STONE, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Package:").color(theme::TEXT_DIM).size(12.0));
            ui.add(egui::TextEdit::singleline(input).desired_width(170.0).hint_text("rustc, gcc, vim, curl..."));
            if ui.button("Install").clicked() && !input.is_empty() {
                let n = input.clone();
                *status = match pkg_mgr.install(&n) {
                    Ok(()) => format!("OK Installed '{}'", n),
                    Err(e) => format!("ERR {:?}", e),
                };
                input.clear();
            }
            if ui.button("Remove").clicked() && !input.is_empty() {
                let n = input.clone();
                *status = match pkg_mgr.remove(&n) {
                    Ok(()) => format!("OK Removed '{}'", n),
                    Err(e) => format!("ERR {:?}", e),
                };
                input.clear();
            }
            if ui.button("Update All").clicked() {
                *status = match pkg_mgr.update_all() {
                    Ok(()) => "OK All updated".to_string(),
                    Err(e) => format!("ERR {:?}", e),
                };
            }
        });
        ui.label(egui::RichText::new("Available: rustc  gcc  vim  curl  git  htop  nano  python  nodejs  docker").color(theme::TEXT_MUTED).size(10.0));
    });

    ui.add_space(12.0);
    fancy_card(ui, "Installed Packages", theme::GREEN, |ui| {
        let pkgs = pkg_mgr.list_installed();
        if pkgs.is_empty() {
            ui.label(egui::RichText::new("No packages installed yet.").color(theme::TEXT_MUTED).size(12.0));
        } else {
            egui::ScrollArea::vertical().id_salt("pkg_list").max_height(320.0).show(ui, |ui| {
                egui::Grid::new("pkg_grid").num_columns(2).spacing([16.0, 5.0]).striped(true).show(ui, |ui| {
                    for p in &pkgs {
                        ui.label(egui::RichText::new(p).color(theme::TEXT).monospace().size(12.0));
                        widgets::pill_badge(ui, "installed", theme::GREEN);
                        ui.end_row();
                    }
                });
            });
        }
    });

    if !status.is_empty() {
        ui.add_space(8.0);
        let color = if status.starts_with("OK") { theme::GREEN_GLOW } else { theme::RUST };
        ui.label(egui::RichText::new(status.as_str()).color(color).size(12.0));
    }
}

fn render_settings(ui: &mut egui::Ui, tab: &mut usize) {
    ui.horizontal(|ui| {
        for (i, (name, accent)) in [
            ("System",   theme::GREEN),
            ("Display",  theme::SAND),
            ("Security", theme::RUST),
            ("About",    theme::STONE),
        ].iter().enumerate() {
            let active = *tab == i;
            let fill = if active { theme::with_alpha(*accent, 22) } else { Color32::TRANSPARENT };
            Frame::none()
                .fill(fill)
                .stroke(Stroke::new(if active { 1.0 } else { 0.0 }, theme::with_alpha(*accent, 80)))
                .rounding(Rounding::same(5.0))
                .inner_margin(Margin::symmetric(10.0, 4.0))
                .show(ui, |ui| {
                    let text = egui::RichText::new(*name).size(12.0)
                        .color(if active { *accent } else { theme::TEXT_DIM });
                    if ui.selectable_label(false, text).clicked() { *tab = i; }
                });
            ui.add_space(2.0);
        }
    });
    ui.add_space(10.0);

    match tab {
        0 => fancy_card(ui, "System Info", theme::GREEN, |ui| {
            widgets::info_row(ui, "Hostname",   "endoros-node-1");
            widgets::info_row(ui, "Arch",       "x86_64");
            widgets::info_row(ui, "Kernel",     "EndorOS 0.1.0 (no_std)");
            widgets::info_row(ui, "Init",       "endoros-init (PID 1)");
            widgets::info_row(ui, "Filesystem", "RamdiskFS (in-memory VFS)");
            widgets::info_row(ui, "Scheduler",  "Round-robin (SchedulerAdapter)");
            widgets::info_row(ui, "IPC",        "Shared memory + pipes");
        }),
        1 => fancy_card(ui, "Display", theme::SAND, |ui| {
            widgets::info_row(ui, "Theme",       "Endor Dark");
            widgets::info_row(ui, "Palette",     "Stone + Moss Green + Sand");
            widgets::info_row(ui, "Rounding",    "8px windows / 5px widgets");
            widgets::info_row(ui, "Animations",  "Minimal (< 100ms)");
            widgets::info_row(ui, "Font",        "System monospace");
            widgets::info_row(ui, "DPI",         "Auto");
            widgets::info_row(ui, "GUI",         "egui 0.29 + eframe");
        }),
        2 => fancy_card(ui, "Security", theme::RUST, |ui| {
            widgets::info_row(ui, "Auth",       "LocalAuthAdapter");
            widgets::info_row(ui, "Crypto",     "XorCryptoAdapter (demo)");
            widgets::info_row(ui, "Isolation",  "Enabled");
            widgets::info_row(ui, "Audit log",  "Enabled");
            widgets::info_row(ui, "Pkg sign",   "Required");
            widgets::info_row(ui, "Policy",     "SecurityPolicy::default_hardened()");
            widgets::info_row(ui, "Timeout",    "3600s");
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                widgets::pill_badge(ui, "Hardened", theme::GREEN);
                ui.add_space(4.0);
                widgets::pill_badge(ui, "Audit ON", theme::GREEN_GLOW);
                ui.add_space(4.0);
                widgets::pill_badge(ui, "Isolation ON", theme::SAND);
            });
        }),
        3 => {
            fancy_card(ui, "About EndorOS", theme::GREEN_GLOW, |ui| {
                ui.label(egui::RichText::new("EndorOS").color(theme::GREEN_GLOW).size(22.0).strong());
                ui.label(egui::RichText::new("Hexagonal OS written in Rust").color(theme::TEXT_DIM).size(12.0));
                ui.add_space(8.0);
                widgets::labeled_separator(ui, "Autor");
                widgets::info_row(ui, "Nombre",      "Leonardo Correa");
                widgets::info_row(ui, "C.I.",        "30889380");
                widgets::info_row(ui, "Universidad", "Universidad Jose Antonio Paez");
                widgets::info_row(ui, "Escuela",     "Escuela de Computacion");
                ui.add_space(8.0);
                widgets::labeled_separator(ui, "Stack");
                widgets::info_row(ui, "Lenguaje",    "Rust (no_std kernel + std UI)");
                widgets::info_row(ui, "GUI",         "egui 0.29 + eframe");
                widgets::info_row(ui, "Arch",        "Hexagonal (Ports & Adapters)");
                widgets::info_row(ui, "Crates",      "kernel / domain / application / infrastructure / ui");
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Inspired by the forest moon of Endor - Star Wars")
                        .color(theme::TEXT_MUTED).size(11.0).italics(),
                );
            });
        }
        _ => {}
    }
}
