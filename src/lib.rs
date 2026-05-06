use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::FsNative;
#[cfg(mobile)]
use mobile::FsNative;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the fs-native APIs.
pub trait FsNativeExt<R: Runtime> {
    fn fs_native(&self) -> &FsNative<R>;
}

impl<R: Runtime, T: Manager<R>> crate::FsNativeExt<R> for T {
    fn fs_native(&self) -> &FsNative<R> {
        self.state::<FsNative<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("fs-light")
        .invoke_handler(tauri::generate_handler![
            commands::read_to_string,
            commands::read
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let fs_native = mobile::init(app, api)?;
            #[cfg(desktop)]
            let fs_native = desktop::init(app, api)?;
            app.manage(fs_native);
            Ok(())
        })
        .build()
}
