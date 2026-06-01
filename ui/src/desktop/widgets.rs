use egui::{Color32, Frame, Margin, Painter, Pos2, Rect, Rounding, Stroke, Ui, Vec2};
use super::theme;

/// Título de panel con punto de color y línea inferior
pub fn title_bar(ui: &mut Ui, title: &str, accent: Color32) {
    Frame::none()
        .fill(theme::TITLEBAR)
        .inner_margin(Margin::symmetric(14.0, 8.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Dot de color
                let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), egui::Sense::hover());
                paint_glowing_dot(ui.painter(), dot_rect.center(), 5.0, accent);
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(title)
                        .color(theme::TEXT)
                        .size(13.5)
                        .strong(),
                );
            });
        });
    // línea coloreada debajo del título
    let rect = ui.min_rect();
    ui.painter().line_segment(
        [Pos2::new(rect.min.x, rect.max.y), Pos2::new(rect.max.x, rect.max.y)],
        Stroke::new(1.0, accent),
    );
}

/// Card de stat con borde de acento y brillo en la esquina superior
pub fn stat_card(ui: &mut Ui, label: &str, value: &str, accent: Color32) {
    let desired = Vec2::new(ui.available_width(), 72.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());

    // Fondo
    ui.painter().rect_filled(rect, Rounding::same(8.0), theme::BG_CARD);
    // Borde
    ui.painter().rect_stroke(rect, Rounding::same(8.0), Stroke::new(1.0, theme::with_alpha(accent, 80)));
    // Borde izquierdo de acento
    ui.painter().rect_filled(
        Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height())),
        Rounding { nw: 8.0, sw: 8.0, ne: 0.0, se: 0.0 },
        accent,
    );
    // Brillo esquina superior derecha
    paint_corner_glow(ui.painter(), rect, accent);

    // Texto
    let inner = rect.shrink2(Vec2::new(12.0, 8.0));
    let inner_left = Rect::from_min_size(inner.min + Vec2::new(4.0, 0.0), inner.size());
    let label_pos = Pos2::new(inner_left.min.x, inner_left.min.y + 4.0);
    let value_pos = Pos2::new(inner_left.min.x, inner_left.min.y + 22.0);

    ui.painter().text(
        label_pos, egui::Align2::LEFT_TOP,
        label,
        egui::FontId::proportional(10.5),
        theme::TEXT_MUTED,
    );
    ui.painter().text(
        value_pos, egui::Align2::LEFT_TOP,
        value,
        egui::FontId::monospace(18.0),
        accent,
    );
}

/// Fila clave/valor alineada
pub fn info_row(ui: &mut Ui, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{:<22}", key))
                .color(theme::TEXT_MUTED)
                .size(12.0)
                .monospace(),
        );
        ui.label(
            egui::RichText::new(value)
                .color(theme::TEXT)
                .size(12.0)
                .monospace(),
        );
    });
}

/// Badge pill con relleno semitransparente
pub fn pill_badge(ui: &mut Ui, text: &str, color: Color32) {
    Frame::none()
        .fill(theme::with_alpha(color, 22))
        .stroke(Stroke::new(1.0, theme::with_alpha(color, 120)))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::symmetric(8.0, 3.0))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).color(color).size(10.5).strong());
        });
}

/// Separador con etiqueta centrada
pub fn labeled_separator(ui: &mut Ui, label: &str) {
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.add(egui::Separator::default().spacing(8.0));
        ui.label(egui::RichText::new(label).color(theme::TEXT_MUTED).size(10.5));
        ui.add(egui::Separator::default().spacing(8.0));
    });
    ui.add_space(2.0);
}

// ── Helpers de pintura ────────────────────────────────────────────────────────

fn paint_glowing_dot(painter: &Painter, center: Pos2, radius: f32, color: Color32) {
    // Halo exterior difuso
    painter.circle_filled(center, radius + 3.0, theme::with_alpha(color, 30));
    painter.circle_filled(center, radius + 1.5, theme::with_alpha(color, 60));
    painter.circle_filled(center, radius, color);
}

fn paint_corner_glow(painter: &Painter, rect: Rect, color: Color32) {
    let center = Pos2::new(rect.max.x - 12.0, rect.min.y + 12.0);
    painter.circle_filled(center, 18.0, theme::with_alpha(color, 12));
    painter.circle_filled(center, 10.0, theme::with_alpha(color, 8));
}
