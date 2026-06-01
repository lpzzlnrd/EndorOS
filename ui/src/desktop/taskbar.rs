use egui::{Frame, Margin, Pos2, Rounding, Stroke, Ui, Vec2};
use super::theme;

#[derive(Clone, PartialEq)]
pub enum AppIcon { Terminal, Files, Processes, Packages, Settings }

impl AppIcon {
    pub fn label(&self) -> &str {
        match self {
            AppIcon::Terminal  => "Terminal",
            AppIcon::Files     => "Files",
            AppIcon::Processes => "Processes",
            AppIcon::Packages  => "Packages",
            AppIcon::Settings  => "Settings",
        }
    }
    pub fn glyph(&self) -> &str {
        match self {
            AppIcon::Terminal  => ">_",
            AppIcon::Files     => "[]",
            AppIcon::Processes => "##",
            AppIcon::Packages  => "<>",
            AppIcon::Settings  => "==",
        }
    }
}

pub struct Taskbar {
    pub apps: Vec<AppIcon>,
    pub open_apps: Vec<AppIcon>,
    pub clock: String,
    pub username: String,
}

impl Taskbar {
    pub fn new() -> Self {
        Self {
            apps: vec![AppIcon::Terminal, AppIcon::Files, AppIcon::Processes, AppIcon::Packages, AppIcon::Settings],
            open_apps: Vec::new(),
            clock: String::new(),
            username: "anonymous".to_string(),
        }
    }

    pub fn update_clock(&mut self) {
        self.clock = chrono::Local::now().format("%H:%M  %d/%m").to_string();
    }

    pub fn render(&mut self, ui: &mut Ui) -> Option<AppIcon> {
        let mut clicked = None;

        let full = ui.max_rect();
        // Línea superior del taskbar como acento sutil
        ui.painter().line_segment(
            [Pos2::new(full.min.x, full.min.y), Pos2::new(full.max.x, full.min.y)],
            Stroke::new(1.0, theme::BORDER_LIT),
        );

        Frame::none()
            .fill(theme::TASKBAR_BG)
            .inner_margin(Margin::symmetric(10.0, 0.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.set_min_height(44.0);
                    ui.set_min_width(ui.available_width());

                    // Logo compacto
                    ui.add_space(6.0);
                    let (logo_rect, _) = ui.allocate_exact_size(Vec2::new(32.0, 32.0), egui::Sense::hover());
                    ui.painter().circle_filled(logo_rect.center(), 13.0, theme::with_alpha(theme::GREEN_DIM, 50));
                    ui.painter().circle_stroke(logo_rect.center(), 13.0, Stroke::new(1.0, theme::GREEN_DIM));
                    ui.painter().text(
                        logo_rect.center(), egui::Align2::CENTER_CENTER,
                        "E", egui::FontId::proportional(16.0), theme::GREEN_GLOW,
                    );
                    ui.add_space(8.0);

                    // Separador vertical
                    let sep_y1 = ui.cursor().min.y + 6.0;
                    let sep_y2 = sep_y1 + 24.0;
                    let sep_x = ui.cursor().min.x;
                    ui.painter().line_segment(
                        [Pos2::new(sep_x, sep_y1), Pos2::new(sep_x, sep_y2)],
                        Stroke::new(1.0, theme::BORDER_LIT),
                    );
                    ui.add_space(10.0);

                    // App buttons
                    for app in &self.apps {
                        let is_open = self.open_apps.contains(app);
                        if taskbar_btn(ui, app.glyph(), app.label(), is_open).clicked() {
                            clicked = Some(app.clone());
                        }
                        ui.add_space(2.0);
                    }

                    // Lado derecho
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);

                        // Reloj
                        ui.label(
                            egui::RichText::new(&self.clock)
                                .color(theme::TEXT_DIM)
                                .size(11.5)
                                .monospace(),
                        );
                        ui.add_space(14.0);

                        // Usuario con dot de estado
                        let is_logged = self.username != "anonymous";
                        let dot_color = if is_logged { theme::GREEN } else { theme::STONE };
                        let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(8.0), egui::Sense::hover());
                        ui.painter().circle_filled(dot_rect.center(), 4.0, dot_color);
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(&self.username)
                                .color(if is_logged { theme::SAND } else { theme::TEXT_DIM })
                                .size(12.0),
                        );
                    });
                });
            });

        clicked
    }
}

fn taskbar_btn(ui: &mut Ui, glyph: &str, label: &str, active: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(
        Vec2::new(label.len() as f32 * 7.5 + 36.0, 36.0),
        egui::Sense::click(),
    );
    let painter = ui.painter();

    if active {
        painter.rect_filled(rect, Rounding::same(5.0), theme::BG_ACTIVE);
        painter.rect_stroke(rect, Rounding::same(5.0), Stroke::new(1.0, theme::with_alpha(theme::GREEN, 80)));
        // Indicador inferior
        painter.rect_filled(
            egui::Rect::from_min_size(
                Pos2::new(rect.center().x - 10.0, rect.max.y - 2.5),
                Vec2::new(20.0, 2.5),
            ),
            Rounding::same(2.0),
            theme::GREEN,
        );
    } else if resp.hovered() {
        painter.rect_filled(rect, Rounding::same(5.0), theme::BG_HOVER);
    }

    let glyph_color = if active { theme::GREEN_GLOW } else if resp.hovered() { theme::TEXT } else { theme::TEXT_DIM };
    let label_color = if active { theme::TEXT } else { theme::TEXT_DIM };
    let center_y = rect.center().y;

    // Glyph + label side by side
    let text_start = rect.min.x + 10.0;
    painter.text(
        Pos2::new(text_start, center_y),
        egui::Align2::LEFT_CENTER,
        glyph,
        egui::FontId::monospace(11.5),
        glyph_color,
    );
    painter.text(
        Pos2::new(text_start + 22.0, center_y),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(12.0),
        label_color,
    );

    resp
}
