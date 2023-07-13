use eframe::epaint::vec2;

/// Entrypoint for desktop.
fn main() -> eframe::Result<()> {
    fmbqtimer::run_native(eframe::NativeOptions {
        initial_window_size: Some(vec2(400.0, 720.0)),
        ..Default::default()
    })
}
