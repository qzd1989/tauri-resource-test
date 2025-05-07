use base64::{engine::general_purpose, Engine as _};
use image::{ImageBuffer, ImageFormat, Rgba};
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use std::{env, io::Cursor};
#[derive(Embed)]
#[folder = "assets/"]
struct Asset;
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Base64Png {
    width: u32,
    height: u32,
    data: String,
}
#[tauri::command]
fn get_test() -> Result<Base64Png, String> {
    let test_png_embeded = Asset::get("test.png").unwrap();
    let test_png_cursor = Cursor::new(test_png_embeded.data);
    let image = image::load(test_png_cursor, ImageFormat::Png).unwrap();
    Ok(Base64Png {
        width: image.width(),
        height: image.height(),
        data: image.to_rgba8().to_base64png().unwrap(),
    })
}
#[tauri::command]
fn unzip_template() -> Result<String, String> {
    let template_zip_embeded = Asset::get("python_project_template.zip").unwrap();
    let template_zip_cursor = Cursor::new(template_zip_embeded.data);
    let current_dir = env::current_dir().unwrap();
    let target_dir = current_dir.join("template");
    zip_extract::extract(template_zip_cursor, &target_dir, true).unwrap();
    Ok(target_dir.to_str().unwrap().to_string())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_test, unzip_template])
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
