use serde::{Serialize, Deserialize};

pub struct Storage {
    backend: Box<dyn StorageBackend>,
}

impl Storage {
    pub(crate) fn new(backend: impl StorageBackend + 'static) -> Self {
        Self {
            backend: Box::new(backend),
        }
    }

    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        serde_json::from_value(self.backend.get(key)?).ok()
    }

    pub fn set<T: Serialize>(&mut self, key: &str, value: T) {
        self.backend.set(key, serde_json::to_value(value).unwrap())
    }

    pub fn clear(&mut self) {
        self.backend.clear()
    }

    pub fn flush(&mut self) {
        self.backend.flush()
    }
}

pub(crate) trait StorageBackend {
    fn get(&self, key: &str) -> Option<serde_json::Value>;

    fn set(&mut self, key: &str, value: serde_json::Value);

    fn clear(&mut self);

    fn flush(&mut self);
}
