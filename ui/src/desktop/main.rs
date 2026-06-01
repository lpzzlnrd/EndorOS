mod app;
mod theme;
mod widgets;
mod terminal;
mod taskbar;
mod file_explorer;
mod process_monitor;

use eframe::NativeOptions;
use egui::ViewportBuilder;

fn main() {
    // Print to stderr so we can see panics even without a console window
    std::panic::set_hook(Box::new(|info| {
        eprintln!("[EndorOS PANIC] {}", info);
        // Also try a message box on Windows
        #[cfg(target_os = "windows")]
        {
            let msg = format!("EndorOS crashed:\n\n{}", info);
            eprintln!("{}", msg);
        }
    }));

    let result = run();
    if let Err(e) = result {
        eprintln!("[EndorOS ERROR] Failed to start: {}", e);
        std::process::exit(1);
    }
}

fn run() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("EndorOS")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_decorations(true)
            .with_resizable(true),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "EndorOS",
        options,
        Box::new(|cc| Ok(Box::new(app::EndorApp::new(cc)))),
    )
}
