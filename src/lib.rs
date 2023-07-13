//! Root module for the app. Doesn't contain any main functions or entrypoints
//! since different platforms have different requirements for that.
//! Platform-specific entrypoints instead use this as a library.

pub mod platform;
mod app;
mod audio_player;
mod sounds;

pub static BUILD_TIME_STR: &str = env!("BUILD_TIME");

pub fn run_native(native_options: eframe::NativeOptions) -> eframe::Result<()> {
    eframe::run_native(
        app::App::NAME,
        native_options,
        Box::new(|ctx| Box::new(app::App::new(ctx))),
    )
}
