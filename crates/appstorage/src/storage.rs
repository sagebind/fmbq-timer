use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct Storage {
    backend: Arc<Mutex<dyn StorageBackend>>,
}

impl Storage {
    pub(crate) fn new(backend: impl StorageBackend + 'static) -> Self {
        Self {
            backend: Arc::new(Mutex::new(backend)),
        }
    }

    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        serde_json::from_value(self.backend.lock().unwrap().get(key)?).ok()
    }

    pub fn set<T: Serialize>(&self, key: &str, value: T) {
        self.backend.lock().unwrap().set(key, serde_json::to_value(value).unwrap())
    }

    /// Remove all properties from storage.
    pub fn clear(&self) {
        self.backend.lock().unwrap().clear()
    }

    /// Flush any modified properties to persistent storage, if necessary.
    ///
    /// The implementation varies across platforms. On some platforms, setting a
    /// property is persisted immediately and flushes are a no-op. You should call
    /// flush anyway to ensure properties are persisted in a cross-platform way.
    pub fn flush(&self) {
        self.backend.lock().unwrap().flush()
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        if Arc::strong_count(&self.backend) == 1 {
            self.flush();
        }
    }
}

pub(crate) trait StorageBackend {
    fn get(&self, key: &str) -> Option<serde_json::Value>;

    fn set(&mut self, key: &str, value: serde_json::Value);

    fn clear(&mut self);

    fn flush(&mut self);
}
