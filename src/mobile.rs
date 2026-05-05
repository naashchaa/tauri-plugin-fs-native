use serde::{de::DeserializeOwned, Deserialize};
use tauri::{
    plugin::{mobile::PluginInvokeError, PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_fs_light);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<FsLight<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.fsnative", "FsNativePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_fs_light)?;
    Ok(FsLight(handle))
}

/// Access to the fs-light APIs.
pub struct FsLight<R: Runtime>(PluginHandle<R>);

#[derive(Deserialize)]
struct CommandOutput {
    content: String,
}

impl<R: Runtime> FsLight<R> {
    pub fn read_to_string(&self, path: String) -> Result<String, PluginInvokeError> {
        Ok(self
            .0
            .run_mobile_plugin::<CommandOutput>("read_to_string", path)?
            .content)
    }
}
