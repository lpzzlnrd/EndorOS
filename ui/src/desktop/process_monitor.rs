use egui::Ui;
use application::ports::process_mgr::ProcessManagerPort;
use infrastructure::process::scheduler::SchedulerAdapter;
use super::theme;

pub struct ProcessMonitor {
    pub new_proc_name: String,
    pub new_proc_priority: u8,
    pub status: String,
    pub kill_input: String,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        Self {
            new_proc_name: String::new(),
            new_proc_priority: 5,
            status: String::new(),
            kill_input: String::new(),
        }
    }

    pub fn render(&mut self, ui: &mut Ui, proc_mgr: &mut SchedulerAdapter) {
        // Launch new process
        egui::Frame::none()
            .fill(theme::BG_WIDGET)
            .inner_margin(egui::Margin::same(8.0))
            .rounding(egui::Rounding::same(4.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Launch:").color(theme::TEXT_DIM).size(12.0));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_proc_name)
                            .hint_text("process name")
                            .desired_width(160.0),
                    );
                    ui.label(egui::RichText::new("Priority:").color(theme::TEXT_DIM).size(12.0));
                    ui.add(egui::DragValue::new(&mut self.new_proc_priority).range(1u8..=255u8));
                    if ui.button("Run").clicked() && !self.new_proc_name.is_empty() {
                        let name = self.new_proc_name.clone();
                        let priority = self.new_proc_priority;
                        match proc_mgr.spawn(&name, priority) {
                            Ok(pid) => self.status = format!("Launched '{}' -> PID {}", name, pid),
                            Err(e) => self.status = format!("Error: {:?}", e),
                        }
                        self.new_proc_name.clear();
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Kill PID:").color(theme::TEXT_DIM).size(12.0));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.kill_input)
                            .hint_text("pid")
                            .desired_width(60.0),
                    );
                    if ui.button("Kill").clicked() {
                        if let Ok(pid) = self.kill_input.trim().parse::<u32>() {
                            match proc_mgr.kill(pid) {
                                Ok(()) => self.status = format!("Killed PID {}", pid),
                                Err(e) => self.status = format!("Error: {:?}", e),
                            }
                        } else {
                            self.status = "Invalid PID".to_string();
                        }
                        self.kill_input.clear();
                    }
                });
            });

        ui.add_space(6.0);

        // Process list
        ui.label(egui::RichText::new("Running Processes").color(theme::SAND).size(12.0).strong());
        ui.separator();

        egui::Grid::new("proc_grid")
            .num_columns(2)
            .spacing([20.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("PID").color(theme::SAND).size(12.0).strong());
                ui.label(egui::RichText::new("NAME").color(theme::SAND).size(12.0).strong());
                ui.end_row();

                let procs = proc_mgr.list_processes();
                if procs.is_empty() {
                    ui.label(egui::RichText::new("â€”").color(theme::TEXT_MUTED).size(12.0));
                    ui.label(egui::RichText::new("No processes running").color(theme::TEXT_MUTED).size(12.0));
                    ui.end_row();
                } else {
                    for (pid, name) in &procs {
                        ui.label(egui::RichText::new(pid.to_string()).color(theme::GREEN_GLOW).monospace().size(12.0));
                        ui.label(egui::RichText::new(name.clone()).color(theme::TEXT).monospace().size(12.0));
                        ui.end_row();
                    }
                }
            });

        if !self.status.is_empty() {
            ui.separator();
            ui.label(egui::RichText::new(&self.status).color(theme::GREEN).size(11.0));
        }
    }
}

