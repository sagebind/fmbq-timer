use crate::app::App;

pub fn run_native(native_options: eframe::NativeOptions) -> eframe::Result<()> {
    eframe::run_native(
        App::NAME,
        native_options,
        Box::new(|ctx| Box::new(App::new(ctx))),
    )
}
