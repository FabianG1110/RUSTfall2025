use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum FetchError {
    Api(String),
    Network(String),
    Io(String),
}

fn fetch_random_dog_image() -> Result<DogImage, FetchError> {
    let url = "https://dog.ceo/api/breeds/image/random";

    let response = ureq::get(url).call().map_err(|e| {
        FetchError::Network(format!("Network request failed: {}", e))
    })?;

    if response.status() != 200 {
        return Err(FetchError::Api(format!("HTTP error: {}", response.status())));
    }

    response.into_json::<DogImage>().map_err(|e| {
        FetchError::Api(format!("Failed to parse JSON: {}", e))
    })
}

fn download_image(url: &str, file_path: &str) -> Result<(), FetchError> {
    let response = ureq::get(url).call().map_err(|e| {
        FetchError::Network(format!("Failed to download image: {}", e))
    })?;

    let mut bytes = response.into_reader();
    let mut file = File::create(file_path).map_err(|e| FetchError::Io(e.to_string()))?;

    std::io::copy(&mut bytes, &mut file)
        .map_err(|e| FetchError::Io(e.to_string()))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Downloader");
    println!("====================\n");

    for i in 1..=5 {
        println!("Fetching dog image #{}", i);

        match fetch_random_dog_image() {
            Ok(dog) => {
                println!("✅ Got image info!");
                println!("🖼️ URL: {}", dog.message);

                let filename = format!("dog_{}.jpg", i);
                match download_image(&dog.message, &filename) {
                    Ok(_) => println!("💾 Saved to {}", filename),
                    Err(e) => println!("❌ Download Error: {:?}", e),
                }
            }
            Err(e) => println!("❌ Fetch Error: {:?}", e),
        }

        println!();
    }

    Ok(())
}
