use image::ImageReader;
use std::path::Path;
use walkdir::WalkDir;
use webp::Encoder;
const QUALITY: f32 = 90.;
fn main() {
    let input_dir = "assets";
    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("png") {
            continue;
        }
        match convert_to_webp(path) {
            Ok(_) => println!("Converted: {}", path.display()),
            Err(e) => eprintln!("Failed to convert {}: {}", path.display(), e),
        }
    }
}
fn convert_to_webp(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(path)?.decode()?;
    let mut webp_path = path.to_path_buf();
    webp_path.set_extension("webp");
    let encoder = Encoder::from_image(&img)?;
    let encoded_webp = encoder.encode(QUALITY);
    std::fs::write(&webp_path, &*encoded_webp)?;
    std::fs::remove_file(path)?;
    Ok(())
}
