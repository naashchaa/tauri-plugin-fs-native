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
use desktop::FsLight;
#[cfg(mobile)]
use mobile::FsLight;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the fs-light APIs.
pub trait FsLightExt<R: Runtime> {
    fn fs_light(&self) -> &FsLight<R>;
}

impl<R: Runtime, T: Manager<R>> crate::FsLightExt<R> for T {
    fn fs_light(&self) -> &FsLight<R> {
        self.state::<FsLight<R>>().inner()
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
            let fs_light = mobile::init(app, api)?;
            #[cfg(desktop)]
            let fs_light = desktop::init(app, api)?;
            app.manage(fs_light);
            Ok(())
        })
        .build()
}
