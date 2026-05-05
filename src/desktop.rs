use std::fs::{self};

use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<FsLight<R>> {
    Ok(FsLight(app.clone()))
}

/// Access to the fs-light APIs.
pub struct FsLight<R: Runtime>(AppHandle<R>);

impl<R: Runtime> FsLight<R> {
    pub fn read_to_string(&self, path: String) -> crate::Result<String> {
        Ok(fs::read_to_string(path)?)
    }

    pub fn read(&self, path: String) -> crate::Result<Vec<u8>> {
        Ok(fs::read(path)?)
    }
}
