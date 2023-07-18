use crate::storage::StorageBackend;
use super::memory::MemoryStorage;
use app_dirs2::AppInfo;
use std::{fs::File, path::PathBuf};

/// Stores data in a JSON file on the local file system.
pub(crate) struct FlatFileStorage {
    path: PathBuf,
    memory: MemoryStorage,
}

impl FlatFileStorage {
    pub fn new(app_info: &crate::AppInfo) -> Self {
        let app_info = AppInfo {
            name: app_info.name,
            author: "",
        };

        let path = app_dirs2::app_root(app_dirs2::AppDataType::UserData, &app_info)
            .unwrap()
            .join("storage.json");

        Self::with_path(path)
    }

    pub fn with_path(path: PathBuf) -> Self {
        let mut storage = Self {
            path,
            memory: MemoryStorage::default(),
        };

        storage.reload();

        storage
    }

    fn reload(&mut self) {
        if let Ok(file) = File::open(&self.path) {
            if let Ok(data_) = serde_json::from_reader(file) {
                self.memory.data = data_;
            }
        }
    }
}

impl StorageBackend for FlatFileStorage {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.memory.get(key)
    }

    fn set(&mut self, key: &str, value: serde_json::Value) {
        self.memory.set(key, value)
    }

    fn clear(&mut self) {
        self.memory.clear();
    }

    fn flush(&mut self) {
        let mut file = File::create(&self.path).unwrap();
        serde_json::to_writer(&mut file, &self.memory.data).unwrap();
        file.sync_data().unwrap();
    }
}
