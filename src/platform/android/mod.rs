//! Provides an entrypoint for Android compatible with `NativeActivity`.

use std::{env, sync::Once};
use winit::platform::android::{
    activity::{AndroidApp, WindowManagerFlags},
    EventLoopBuilderExtAndroid,
};

use crate::PlatformContext;

pub mod audio_player;

#[no_mangle]
pub fn android_main(app: AndroidApp) {
    init_logging();

    log::info!("screen density: {:?}", app.config().density());
    log::info!("content rect: {:?}", app.content_rect());

    // Trick egui to persist storage in our dedicated Android storage location.
    if let Some(path) = app.internal_data_path() {
        log::info!("internal data path: {:?}", path);
        env::set_var("HOME", path.to_str().unwrap());
        env::set_var("XDG_DATA_HOME", path.to_str().unwrap());
    }

    // Ask Android to keep the screen on while this app is visible. This is very
    // helpful for ensuring that our timer thread won't be hindered and that the
    // timer state is clearly visible while in use.
    //
    // Also confirm that our app layout accounts for painting under the status and
    // navigation bars.
    app.set_window_flags(
        WindowManagerFlags::KEEP_SCREEN_ON
            | WindowManagerFlags::LAYOUT_IN_SCREEN
            | WindowManagerFlags::LAYOUT_INSET_DECOR,
        WindowManagerFlags::empty(),
    );

    let config = app.config();
    let app_clone = app.clone();

    let native_options = eframe::NativeOptions {
        default_theme: match config.ui_mode_night() {
            ndk::configuration::UiModeNight::Yes => eframe::Theme::Dark,
            ndk::configuration::UiModeNight::No => eframe::Theme::Light,
            ndk::configuration::UiModeNight::Any => eframe::Theme::Dark,
        },
        // run_and_return: false, // https://github.com/rust-windowing/winit/issues/2706
        run_and_return: true,
        event_loop_builder: Some(Box::new(move |builder| {
            builder.with_android_app(app_clone);
        })),
        ..Default::default()
    };

    let platform_ctx = PlatformContext {
        storage: appstorage::open("fmbqtimer"),
        android_app: app,
    };

    if let Err(e) = crate::run_native(native_options, platform_ctx) {
        log::error!("eframe exited with error: {}", e);
    }
}

fn init_logging() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Debug)
                .with_tag("fmbqtimer"),
        );

        log_panics::init();
    });
}
