//! Provides an entrypoint for Android compatible with `NativeActivity`.

use std::{env, sync::OnceLock};
use winit::platform::android::{
    activity::{AndroidApp, WindowManagerFlags},
    EventLoopBuilderExtAndroid,
};

pub static APP: OnceLock<AndroidApp> = OnceLock::new();

pub mod audio_player;

#[no_mangle]
pub fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default()
        .with_max_level(log::LevelFilter::Debug)
        .with_tag("fmbqtimer"));

    APP.set(app.clone()).expect("main called more than once");
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

    let mut native_options = eframe::NativeOptions::default();
    native_options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    // https://github.com/rust-windowing/winit/issues/2706
    native_options.run_and_return = false;

    crate::run_native(native_options).unwrap();
}
