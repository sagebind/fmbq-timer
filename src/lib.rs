mod run;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    let mut native_options = eframe::NativeOptions::default();
    native_options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;

        builder.with_android_app(app);
    }));

    run::run_native(native_options).unwrap();
}
