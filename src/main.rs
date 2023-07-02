use eframe::epaint::vec2;

shadow_rs::shadow!(build);

mod app;
mod platform;
mod run;

/// Entrypoint for desktop.
fn main() -> eframe::Result<()> {
    run::run_native(eframe::NativeOptions {
        initial_window_size: Some(vec2(400.0, 720.0)),
        ..Default::default()
    })
}
