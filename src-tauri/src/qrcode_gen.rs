use base64::Engine;
use image::Luma;
use qrcode::QrCode;
use std::io::Cursor;

pub fn generate_qr_data_url(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let code = QrCode::new(content.as_bytes())?;
    let img = code.render::<Luma<u8>>().quiet_zone(true).build();

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    img.write_to(&mut cursor, image::ImageFormat::Png)?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(&buf);
    Ok(format!("data:image/png;base64,{}", b64))
}

pub fn generate_qr_png_bytes(content: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let code = QrCode::new(content.as_bytes())?;
    let img = code.render::<Luma<u8>>().quiet_zone(true).build();

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    img.write_to(&mut cursor, image::ImageFormat::Png)?;
    Ok(buf)
}
