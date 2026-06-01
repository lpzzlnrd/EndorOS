use egui::{ScrollArea, Ui};
use application::ports::filesystem::FileSystemPort;
use infrastructure::fs::ramdisk::RamdiskAdapter;
use super::theme;

pub struct FileExplorer {
    pub current_path: String,
    pub selected: Option<String>,
    pub file_content: Option<String>,
    pub new_file_name: String,
    pub new_file_content: String,
    pub show_new_file: bool,
    pub status: String,
}

impl FileExplorer {
    pub fn new() -> Self {
        Self {
            current_path: "/".to_string(),
            selected: None,
            file_content: None,
            new_file_name: String::new(),
            new_file_content: String::new(),
            show_new_file: false,
            status: String::new(),
        }
    }

    pub fn render(&mut self, ui: &mut Ui, fs: &mut RamdiskAdapter) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Path:").color(theme::TEXT_DIM).size(12.0));
            ui.label(egui::RichText::new(&self.current_path).color(theme::SAND).size(12.0).monospace());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button("+ New File").clicked() {
                    self.show_new_file = !self.show_new_file;
                }
            });
        });

        ui.separator();

        if self.show_new_file {
            egui::Frame::none()
                .fill(theme::BG_WIDGET)
                .inner_margin(egui::Margin::same(8.0))
                .rounding(egui::Rounding::same(4.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Name:").color(theme::TEXT_DIM).size(12.0));
                        ui.text_edit_singleline(&mut self.new_file_name);
                    });
                    ui.label(egui::RichText::new("Content:").color(theme::TEXT_DIM).size(12.0));
                    ui.text_edit_multiline(&mut self.new_file_content);
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() && !self.new_file_name.is_empty() {
                            let path = if self.current_path.ends_with('/') {
                                format!("{}{}", self.current_path, self.new_file_name)
                            } else {
                                format!("{}/{}", self.current_path, self.new_file_name)
                            };
                            match fs.write(&path, self.new_file_content.as_bytes()) {
                                Ok(()) => {
                                    self.status = format!("Created '{}'", path);
                                    self.new_file_name.clear();
                                    self.new_file_content.clear();
                                    self.show_new_file = false;
                                }
                                Err(e) => self.status = format!("Error: {:?}", e),
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_new_file = false;
                        }
                    });
                });
            ui.add_space(4.0);
        }

        ui.columns(2, |cols| {
            // Left: directory tree
            cols[0].label(egui::RichText::new("Files").color(theme::SAND).size(12.0).strong());
            cols[0].separator();
            ScrollArea::vertical()
                .id_salt("file_list")
                .max_height(300.0)
                .show(&mut cols[0], |ui| {
                    // Parent dir button
                    if self.current_path != "/" {
                        if ui.selectable_label(false, egui::RichText::new("..").color(theme::TEXT_DIM).monospace()).clicked() {
                            let parent = std::path::Path::new(&self.current_path)
                                .parent()
                                .map(|p| p.to_string_lossy().replace('\\', "/"))
                                .unwrap_or("/".to_string());
                            self.current_path = if parent.is_empty() { "/".to_string() } else { parent };
                            self.selected = None;
                            self.file_content = None;
                        }
                    }

                    match fs.list(&self.current_path) {
                        Ok(entries) => {
                            for entry in entries {
                                let is_dir = fs.list(&entry).is_ok();
                                let name = entry.split('/').last().unwrap_or(&entry);
                                let display = if is_dir {
                                    egui::RichText::new(format!("[{}]", name))
                                        .color(theme::GREEN_GLOW)
                                        .monospace()
                                        .size(12.0)
                                } else {
                                    egui::RichText::new(name)
                                        .color(theme::TEXT)
                                        .monospace()
                                        .size(12.0)
                                };
                                let selected = self.selected.as_deref() == Some(&entry);
                                if ui.selectable_label(selected, display).clicked() {
                                    if is_dir {
                                        self.current_path = entry.clone();
                                        self.selected = None;
                                        self.file_content = None;
                                    } else {
                                        self.selected = Some(entry.clone());
                                        match fs.read(&entry) {
                                            Ok(data) => {
                                                self.file_content = Some(
                                                    String::from_utf8_lossy(&data).to_string()
                                                );
                                            }
                                            Err(e) => {
                                                self.file_content = Some(format!("<Error: {:?}>", e));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            ui.colored_label(egui::Color32::from_rgb(200, 80, 80), format!("Error: {:?}", e));
                        }
                    }
                });

            // Right: file content viewer
            cols[1].label(egui::RichText::new("Content").color(theme::SAND).size(12.0).strong());
            cols[1].separator();
            if let Some(content) = &self.file_content {
                ScrollArea::vertical()
                    .id_salt("file_content")
                    .max_height(300.0)
                    .show(&mut cols[1], |ui| {
                        ui.label(
                            egui::RichText::new(content)
                                .color(theme::TERM_TEXT)
                                .monospace()
                                .size(12.0),
                        );
                    });
            } else {
                cols[1].colored_label(theme::TEXT_MUTED, "Select a file to view its content.");
            }
        });

        if !self.status.is_empty() {
            ui.separator();
            ui.label(egui::RichText::new(&self.status).color(theme::GREEN).size(11.0));
        }
    }
}

