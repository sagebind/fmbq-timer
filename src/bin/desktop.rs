use eframe::epaint::vec2;
use fmbqtimer::PlatformContext;

/// Entrypoint for desktop.
fn main() -> eframe::Result<()> {
    #[cfg(not(target_os = "android"))]
    {
        env_logger::init();

        let platform_ctx = PlatformContext {
            storage: appstorage::open("fmbqtimer"),
        };

        fmbqtimer::run_native(
            eframe::NativeOptions {
                initial_window_size: Some(vec2(400.0, 720.0)),
                ..Default::default()
            },
            platform_ctx,
        )
    }

    #[cfg(target_os = "android")]
    {
        Ok(())
    }
}
