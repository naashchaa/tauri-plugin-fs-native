use tauri::{command, Runtime};

use crate::FsNativeExt;
use crate::Result;

#[command]
pub(crate) async fn read_to_string<R: Runtime>(
    app: tauri::AppHandle<R>,
    path: String,
) -> Result<String> {
    Ok(app.fs_native().read_to_string(path)?)
}

#[command]
pub(crate) async fn read<R: Runtime>(app: tauri::AppHandle<R>, path: String) -> Result<Vec<u8>> {
    Ok(app.fs_native().read(path)?)
}
