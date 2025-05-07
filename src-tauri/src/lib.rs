use base64::{engine::general_purpose, Engine as _};
use image::{ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Manager as _};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Base64Png {
    width: u32,
    height: u32,
    data: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(app: AppHandle) -> Result<Base64Png, String> {
    let test_png = app
        .path()
        .resolve("assets/test.png", BaseDirectory::Resource)
        .unwrap();
    let img = image::open(&test_png).unwrap();
    let rgba_img = img.to_rgba8();
    let base64_png = Base64Png {
        width: rgba_img.width(),
        height: rgba_img.height(),
        data: rgba_img.to_base64png().unwrap(),
    };
    Ok(base64_png)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub trait ImageBufferRgbaExt {
    fn to_base64png(&self) -> Result<String, String>;
}
impl ImageBufferRgbaExt for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn to_base64png(&self) -> Result<String, String> {
        let mut bytes = Vec::new();
        if let Err(error) = self.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        ) {
            return Err(error.to_string());
        }
        Ok(format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(bytes)
        ))
    }
}
