use crate::storage::StorageBackend;
use std::collections::HashMap;

/// An ephemeral storage backend that only stores data in memory.
///
/// This is the fallback backend if no persistence mechanism is available. It is
/// also useful as a basis for other implementations.
#[derive(Debug, Default)]
pub(crate) struct MemoryStorage {
    pub(crate) data: HashMap<String, serde_json::Value>,
}

impl StorageBackend for MemoryStorage {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: &str, value: serde_json::Value) {
        self.data.insert(key.to_owned(), value);
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn flush(&mut self) {}
}
