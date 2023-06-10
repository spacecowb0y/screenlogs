// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use mac_address::get_mac_address;
use reqwest::multipart;
use reqwest::Client;
use screenshots::Screen;
use std::time::{SystemTime, UNIX_EPOCH};

const API_ENDPOINT: &str = "http://0.0.0.0:3000/upload";

#[tauri::command]
fn upload_screenshots() {
    tauri::async_runtime::spawn(async move {
        if let Err(err) = upload_screenshots_internal().await {
            eprintln!("Error: {}", err);
        }
    });
}

async fn upload_screenshots_internal() -> Result<(), Box<dyn std::error::Error>> {
    let screens = Screen::all()?;
    let client = Client::new();
    let form = create_multipart_form(&screens)?;

    let request = client
        .post(API_ENDPOINT)
        .multipart(form.try_into()?)
        .send()
        .await?;

    let status = request.status();
    println!("Screenshots uploaded with status: {}", status);

    let body = request.bytes().await?;
    println!("Response Body: {:?}", body);

    Ok(())
}

fn create_multipart_form(
    screens: &[Screen],
) -> Result<multipart::Form, Box<dyn std::error::Error>> {
    let mut form = multipart::Form::new();
    form = add_mac_address_field(form)?;
    for screen in screens {
        let image_data = capture_screenshot(screen)?;
        let file_name = generate_file_name(screen)?;
        let part = create_multipart_part(image_data, file_name)?;
        form = add_multipart_part(form, part);
    }
    Ok(form)
}

fn add_mac_address_field(
    form: multipart::Form,
) -> Result<multipart::Form, Box<dyn std::error::Error>> {
    let mac_address = get_mac_address()?.ok_or("mac address error")?.to_string();
    Ok(form.text("mac_address", mac_address))
}

fn capture_screenshot(screen: &Screen) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let image = screen.capture()?;
    image.to_png().map_err(|e| e.into())
}

fn generate_file_name(screen: &Screen) -> Result<String, Box<dyn std::error::Error>> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let file_name = format!(
        "screenshot-{}-{}.png",
        screen.display_info.id,
        duration.as_secs()
    );
    Ok(file_name)
}

fn create_multipart_part(
    image_data: Vec<u8>,
    file_name: String,
) -> Result<multipart::Part, Box<dyn std::error::Error>> {
    let part = multipart::Part::bytes(image_data)
        .file_name(file_name)
        .mime_str("image/png")?;
    Ok(part)
}

fn add_multipart_part(form: multipart::Form, part: multipart::Part) -> multipart::Form {
    form.part("images", part)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![upload_screenshots])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
