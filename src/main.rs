use image::codecs::webp::WebPEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType};
use std::env;
use std::fs;
use std::io::BufWriter;

fn main() {
    // Get folder path and quality from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <folder_path> <quality (0-100)>", args[0]);
        return;
    }

    let folder_path = &args[1];
    let quality: u8 = args[2]
        .parse()
        .expect("Quality must be an integer between 0 and 100");

    // Ensure quality is within valid range
    if quality > 100 {
        eprintln!("Quality must be between 0 and 100.");
        return;
    }

    // Read all files in the folder
    let paths = fs::read_dir(folder_path).expect("Failed to read the directory");

    for path in paths {
        let path = path.expect("Failed to read path").path();

        // Only process files that are images (jpg, jpeg, png)
        if let Some(extension) = path.extension() {
            let ext_str = extension.to_str().unwrap_or("").to_lowercase();
            if ext_str == "jpg" || ext_str == "jpeg" || ext_str == "png" {
                // Open the image
                let img = ImageReader::open(&path)
                    .expect("Failed to open image")
                    .decode()
                    .expect("Failed to decode image");

                // Prepare the new file path with .webp extension
                let new_path = path.with_extension("webp");

                // Create the output file
                let output_file =
                    std::fs::File::create(&new_path).expect("Failed to create output file");
                let writer = BufWriter::new(output_file);

                // Use the WebPEncoder to encode the image with specified quality
                let  encoder = WebPEncoder::new_lossless(writer);

                // Encode the image. Use the appropriate color type.
                encoder
                    .encode(&img.to_rgba8(), img.width(), img.height(), ColorType::Rgba8)
                    .expect("Failed to encode to WebP");

                // Delete the original file
                fs::remove_file(&path).expect("Failed to delete the original image");
                println!("Converted and deleted: {:?}", path);
            }
        }
    }
}

// Run command
// cargo run -- <folder_path> <quality>
// for example
// cargo run -- ./src/Assets 70
