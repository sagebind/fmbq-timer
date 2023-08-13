pub(crate) mod memory;

#[cfg(target_arch = "wasm32")]
pub(crate) mod web;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod file;

#[cfg(windows)]
pub(crate) mod windows_storage;
