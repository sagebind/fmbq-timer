//! A key-value store for persisting application preferences and data.

pub use storage::Storage;

mod backends;
mod storage;

pub fn open(app_name: &'static str) -> Storage {
    OpenOptions::new(app_name).open()
}

pub struct OpenOptions {
    app_name: &'static str,
    use_winrt_storage: bool,
}

impl OpenOptions {
    pub fn new(app_name: &'static str) -> Self {
        Self {
            app_name,
            use_winrt_storage: false,
        }
    }

    pub fn open(&self) -> Storage {
        let app_info = AppInfo { name: self.app_name };

        #[cfg(windows)]
        {
            if self.use_winrt_storage {
                return Storage::new(backends::windows_storage::WindowsStorage::local_settings().unwrap());
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        return Storage::new(backends::file::FlatFileStorage::new(&app_info));

        Storage::new(backends::memory::MemoryStorage::default())
    }
}

struct AppInfo {
    name: &'static str,
}
