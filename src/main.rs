use eframe::epaint::vec2;

mod app;
mod platform;
mod run;

/// Entrypoint for desktop.
fn main() -> eframe::Result<()> {
    run::run_native(eframe::NativeOptions {
        initial_window_size: Some(vec2(400.0, 600.0)),
        ..Default::default()
    })
}
