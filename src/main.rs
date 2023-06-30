mod run;

/// Entrypoint for desktop.
fn main() -> eframe::Result<()> {
    run::run_native(eframe::NativeOptions::default())
}
