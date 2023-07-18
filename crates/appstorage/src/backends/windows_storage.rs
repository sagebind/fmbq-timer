use crate::storage::StorageBackend;
use std::convert::TryFrom;
use windows::{
    core::{IInspectable, Result, HSTRING},
    Storage::{ApplicationData, ApplicationDataContainer},
};

pub(crate) struct WindowsStorage {
    container: ApplicationDataContainer,
}

impl WindowsStorage {
    pub(crate) fn local_settings() -> Result<Self> {
        let application_data = ApplicationData::Current()?;
        let container = application_data.LocalSettings()?;

        Ok(Self { container })
    }
}

impl StorageBackend for WindowsStorage {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.container
            .Values()
            .ok()?
            .Lookup(&key.into())
            .ok()
            .map(deserialize)
    }

    fn set(&mut self, key: &str, value: serde_json::Value) {
        self.container
            .Values()
            .unwrap()
            .Insert(&key.into(), &serialize(&value))
            .ok();
    }

    fn clear(&mut self) {
        self.container.Values().unwrap().Clear().unwrap();
    }

    fn flush(&mut self) {}
}

impl Drop for WindowsStorage {
    fn drop(&mut self) {
        self.container.Close().unwrap();
    }
}

fn serialize(value: &serde_json::Value) -> IInspectable {
    let json_string = serde_json::to_string(value).unwrap();
    IInspectable::try_from(HSTRING::from(json_string)).unwrap()
}

fn deserialize(value: IInspectable) -> serde_json::Value {
    let json_string = String::try_from(HSTRING::try_from(value).unwrap()).unwrap();
    serde_json::from_str(&json_string).unwrap()
}
