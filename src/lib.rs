//! Root module for the app. Doesn't contain any main functions or entrypoints
//! since different platforms have different requirements for that.
//! Platform-specific entrypoints instead use this as a library.

pub mod platform;
mod app;
mod audio;
mod audio_player;
mod sounds;

pub static BUILD_TIME_STR: &str = env!("BUILD_TIME");

pub struct PlatformContext {
    pub storage: appstorage::Storage,

    #[cfg(target_os = "android")]
    pub android_app: winit::platform::android::activity::AndroidApp,
}

pub fn run_native(native_options: eframe::NativeOptions, platform_ctx: PlatformContext) -> eframe::Result<()> {
    eframe::run_native(
        app::App::NAME,
        native_options,
        Box::new(move |ctx| Box::new(app::App::new(ctx, platform_ctx))),
    )
}
