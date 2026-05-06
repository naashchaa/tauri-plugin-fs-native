use std::fs::{self};

use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<FsNative<R>> {
    Ok(FsNative(app.clone()))
}

/// Access to the fs-light APIs.
pub struct FsNative<R: Runtime>(AppHandle<R>);

impl<R: Runtime> FsNative<R> {
    pub fn read_to_string(&self, path: String) -> crate::Result<String> {
        Ok(fs::read_to_string(path)?)
    }

    pub fn read(&self, path: String) -> crate::Result<Vec<u8>> {
        Ok(fs::read(path)?)
    }
}
