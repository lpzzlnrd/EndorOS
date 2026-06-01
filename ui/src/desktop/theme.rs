use egui::{Color32, Rounding, Stroke, Style, Visuals};

// ── Fondos ──────────────────────────────────────────────────────────────────
pub const BG_BASE: Color32        = Color32::from_rgb(12, 14, 16);
pub const BG_PANEL: Color32       = Color32::from_rgb(20, 23, 27);
pub const BG_WIDGET: Color32      = Color32::from_rgb(28, 32, 38);
pub const BG_HOVER: Color32       = Color32::from_rgb(36, 42, 50);
pub const BG_ACTIVE: Color32      = Color32::from_rgb(44, 52, 62);
pub const BG_CARD: Color32        = Color32::from_rgb(24, 28, 34);

// ── Accentos Endor ───────────────────────────────────────────────────────────
pub const GREEN: Color32          = Color32::from_rgb(82, 160, 100);   // verde bosque vivo
pub const GREEN_DIM: Color32      = Color32::from_rgb(52, 110, 68);    // verde musgo
pub const GREEN_GLOW: Color32     = Color32::from_rgb(120, 200, 140);  // verde brillante
pub const SAND: Color32           = Color32::from_rgb(195, 175, 130);  // arena cálida
pub const STONE: Color32          = Color32::from_rgb(120, 130, 145);  // gris piedra azulado
pub const RUST: Color32           = Color32::from_rgb(180, 90, 70);    // rojo apagado (errores)
pub const AMBER: Color32          = Color32::from_rgb(200, 160, 60);   // ámbar (warnings)

// ── Texto ────────────────────────────────────────────────────────────────────
pub const TEXT: Color32           = Color32::from_rgb(225, 220, 210);
pub const TEXT_DIM: Color32       = Color32::from_rgb(150, 158, 168);
pub const TEXT_MUTED: Color32     = Color32::from_rgb(72, 82, 95);

// ── Bordes ───────────────────────────────────────────────────────────────────
pub const BORDER: Color32         = Color32::from_rgb(36, 42, 52);
pub const BORDER_LIT: Color32     = Color32::from_rgb(56, 66, 80);
pub const BORDER_GREEN: Color32   = Color32::from_rgb(52, 90, 62);

// ── Barras y chrome ──────────────────────────────────────────────────────────
pub const TITLEBAR: Color32       = Color32::from_rgb(16, 19, 23);
pub const SIDEBAR_BG: Color32     = Color32::from_rgb(14, 17, 21);
pub const TASKBAR_BG: Color32     = Color32::from_rgb(10, 12, 15);

// ── Terminal ─────────────────────────────────────────────────────────────────
pub const TERM_BG: Color32        = Color32::from_rgb(8, 10, 12);
pub const TERM_GREEN: Color32     = Color32::from_rgb(80, 200, 110);
pub const TERM_TEXT: Color32      = Color32::from_rgb(200, 210, 195);
pub const TERM_CMD: Color32       = Color32::from_rgb(140, 220, 160);
pub const TERM_ERR: Color32       = Color32::from_rgb(220, 90, 80);
pub const TERM_WARN: Color32      = Color32::from_rgb(210, 170, 70);
pub const TERM_INFO: Color32      = Color32::from_rgb(100, 170, 210);

// ── Helpers ───────────────────────────────────────────────────────────────────
pub fn with_alpha(c: Color32, a: u8) -> Color32 {
    Color32::from_rgba_premultiplied(
        (c.r() as u16 * a as u16 / 255) as u8,
        (c.g() as u16 * a as u16 / 255) as u8,
        (c.b() as u16 * a as u16 / 255) as u8,
        a,
    )
}

pub fn apply_no_font(ctx: &egui::Context) {
    let mut style = Style::default();
    style.visuals = Visuals {
        dark_mode: true,
        override_text_color: Some(TEXT),
        panel_fill: BG_PANEL,
        window_fill: BG_PANEL,
        extreme_bg_color: BG_BASE,
        faint_bg_color: BG_WIDGET,
        code_bg_color: TERM_BG,
        warn_fg_color: AMBER,
        error_fg_color: RUST,
        hyperlink_color: GREEN_GLOW,
        selection: egui::style::Selection {
            bg_fill: with_alpha(GREEN_DIM, 60),
            stroke: Stroke::new(1.0, GREEN),
        },
        window_rounding: Rounding::same(8.0),
        window_stroke: Stroke::new(1.0, BORDER_LIT),
        window_shadow: egui::Shadow {
            offset: egui::vec2(0.0, 8.0),
            blur: 24.0,
            spread: 0.0,
            color: Color32::from_black_alpha(120),
        },
        popup_shadow: egui::Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 12.0,
            spread: 0.0,
            color: Color32::from_black_alpha(80),
        },
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: BG_WIDGET,
                weak_bg_fill: BG_WIDGET,
                bg_stroke: Stroke::new(1.0, BORDER),
                rounding: Rounding::same(5.0),
                fg_stroke: Stroke::new(1.0, TEXT_DIM),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: BG_WIDGET,
                weak_bg_fill: BG_WIDGET,
                bg_stroke: Stroke::new(1.0, BORDER),
                rounding: Rounding::same(5.0),
                fg_stroke: Stroke::new(1.0, TEXT),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: BG_HOVER,
                weak_bg_fill: BG_HOVER,
                bg_stroke: Stroke::new(1.0, GREEN_DIM),
                rounding: Rounding::same(5.0),
                fg_stroke: Stroke::new(1.5, TEXT),
                expansion: 1.0,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: BG_ACTIVE,
                weak_bg_fill: BG_ACTIVE,
                bg_stroke: Stroke::new(1.5, GREEN),
                rounding: Rounding::same(5.0),
                fg_stroke: Stroke::new(2.0, GREEN_GLOW),
                expansion: 1.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: BG_ACTIVE,
                weak_bg_fill: BG_ACTIVE,
                bg_stroke: Stroke::new(1.0, GREEN),
                rounding: Rounding::same(5.0),
                fg_stroke: Stroke::new(1.5, TEXT),
                expansion: 0.0,
            },
        },
        ..Visuals::dark()
    };
    style.spacing.item_spacing        = egui::vec2(6.0, 5.0);
    style.spacing.window_margin       = egui::Margin::same(12.0);
    style.spacing.button_padding      = egui::vec2(10.0, 5.0);
    style.spacing.menu_margin         = egui::Margin::same(6.0);
    style.spacing.indent              = 16.0;
    style.spacing.scroll.bar_width    = 4.0;
    ctx.set_style(style);
}
