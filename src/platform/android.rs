use std::sync::OnceLock;
use winit::platform::android::{activity::AndroidApp, EventLoopBuilderExtAndroid};

pub static APP: OnceLock<AndroidApp> = OnceLock::new();

#[no_mangle]
pub fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Debug));

    APP.set(app.clone()).expect("main called more than once");
    log::info!("screen density: {:?}", app.config().density());
    log::info!("content rect: {:?}", app.content_rect());

    let mut native_options = eframe::NativeOptions::default();
    native_options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    crate::run::run_native(native_options).unwrap();
}
