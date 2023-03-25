use base64::{engine::general_purpose, Engine as _};
use image::{io::Reader as ImageReader, DynamicImage, RgbaImage};
use reqwest;
use serde_json::{json, Value};
use std::fs;
use std::{borrow::Cow, io::Cursor};

fn ensure_dir_exists(path: &str) -> std::io::Result<()> {
    let dir_path = std::path::Path::new(path).parent().unwrap();
    fs::create_dir_all(dir_path)
}

/// Sends a request to generate an image based on the given text using the Kakao Brain API.
///
/// # Arguments
///
/// * `text` - a string slice containing the prompt to generate the image from
/// * `batch_size` - a usize specifying the number of images to generate per request (1 to 8)
/// * `api_key` - a string slice containing the API key to use for the generation request
///
/// # Returns
///
/// This function returns a `Result` with the API response as a `Value` on success, or a `reqwest::Error` on failure.
async fn t2i(text: &str, batch_size: usize, api_key: &str) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.kakaobrain.com/v1/inference/karlo/t2i")
        .header("Authorization", format!("KakaoAK {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "prompt": {
                "text": text,
                "batch_size": batch_size
            }
        }))
        .send()
        .await?;

    println!("Generating image based on text...");
    let response: Value = res.json().await?;
    Ok(response)
}

/// Decodes a base64-encoded string into an RgbaImage.
///
/// # Arguments
///
/// * `base64_string` - a string slice containing the base64-encoded image data
///
/// # Returns
///
/// This function returns an `RgbaImage`.
pub fn string_to_image(base64_string: &str) -> RgbaImage {
    let img_data = general_purpose::STANDARD.decode(base64_string).unwrap();
    let img = ImageReader::new(Cursor::new(img_data))
        .with_guessed_format()
        .expect("Failed to guess image format")
        .decode()
        .expect("Failed to decode image")
        .to_rgba8();
    img
}

/// Asynchronously generates an image based on the given prompt and saves it to a file
///
/// # Arguments
///
/// * `prompt` - a string slice containing the prompt to generate the image from
/// * `output_prefix` - a string slice containing the prefix to use when generating the output file name
/// * `api_key` - a string slice containing the API key to use for the generation request
/// * `batch_size` - an optional `usize` specifying the number of images to generate per request (1 to 8)
///
/// # Returns
///
/// This function returns a `Result` with an empty tuple `()` on success, or an error wrapped in a `Box<dyn std::error::Error>` on failure.
///
/// # Examples
///
/// ```
/// use karlo_rs::generate_image;
///
/// let prompt = "A beautiful sunset over the ocean";
/// let output_prefix = "sunset";
/// let api_key = "my_api_key";
///
/// // Generate a single image with the default batch size
/// let result = generate_image(prompt, output_prefix, api_key, None).await;
///
/// assert!(result.is_ok());
/// ```
pub async fn generate_image(
    prompt: &str,
    output_prefix: &str,
    api_key: &str,
    batch_size: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let batch_size = batch_size.unwrap_or(1);

    let response = t2i(prompt, batch_size, api_key).await?;

    for (index, image_data) in response["images"].as_array().unwrap().iter().enumerate() {
        let image_base64 = image_data["image"].as_str().unwrap();
        let result = string_to_image(image_base64);
        let output_path = Cow::from(format!("{}_{}.png", output_prefix, index + 1));
        ensure_dir_exists(&output_path)?;
        result.save(&*output_path)?;

        println!("Generated image saved to {}", output_path);
    }

    Ok(())
}

/** GENERATE VARIATIONS **/

/// Sends a request to generate variations of an input image using the Kakao Brain API.
///
/// # Arguments
///
/// * `image_base64` - a string slice containing the base64-encoded image data
/// * `batch_size` - a usize specifying the number of variations to generate per request (1 to 8)
/// * `api_key` - a string slice containing the API key to use for the generation request
///
/// # Returns
///
/// This function returns a `Result` with the API response as a `Value` on success, or a `reqwest::Error` on failure.
async fn variations(
    image_base64: &str,
    batch_size: usize,
    api_key: &str,
) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.kakaobrain.com/v1/inference/karlo/variations")
        .header("Authorization", format!("KakaoAK {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "prompt": {
                "image": image_base64,
                "batch_size": batch_size
            }
        }))
        .send()
        .await?;

    println!("Generating variations...");
    let response: Value = res.json().await?;
    Ok(response)
}

/// Encodes an image as a base64-encoded string.
///
/// # Arguments
///
/// * `img` - a reference to a `DynamicImage`
///
/// # Returns
///
/// String of base64-encoded string
fn image_to_base64_string(img: &DynamicImage) -> String {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    img.write_to(&mut cursor, image::ImageOutputFormat::Png)
        .expect("Failed to write image to buffer");

    general_purpose::STANDARD.encode(buffer)
}

/// Asynchronously generates image variations based on the images found in the input directory and saves them to files
///
/// # Arguments
///
/// * `input_path` - a string slice containing the path to the input directory containing the images to generate variations from
/// * `output_prefix` - a string slice containing the prefix to use when generating the output file names
/// * `api_key` - a string slice containing the API key to use for the generation requests
/// * `batch_size` - an optional `usize` specifying the number of images to generate per request (1 to 8)
///
/// # Returns
///
/// This function returns a `Result` with an empty tuple `()` on success, or an error wrapped in a `Box<dyn std::error::Error>` on failure.
///
/// # Examples
///
/// ```
/// use my_cool_library::generate_variations;
///
/// let input_path = "/path/to/input/dir";
/// let output_prefix = "output";
/// let api_key = "my_api_key";
///
/// // Generate variations for all images in the input directory with the default batch size
/// let result = generate_variations(input_path, output_prefix, api_key, None).await;
///
/// assert!(result.is_ok());
/// ```
pub async fn generate_variations(
    input_path: &str,
    output_prefix: &str,
    api_key: &str,
    batch_size: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let batch_size = batch_size.unwrap_or(1);

    let input_image = image::open(input_path)?;
    let img_base64 = image_to_base64_string(&input_image);

    let response = variations(&img_base64, batch_size, api_key).await?;

    for (index, image_data) in response["images"].as_array().unwrap().iter().enumerate() {
        let image_base64 = image_data["image"].as_str().unwrap();
        let result = string_to_image(image_base64);
        let output_path = Cow::from(format!("{}_{}.png", output_prefix, index + 1));
        ensure_dir_exists(&output_path)?;
        result.save(&*output_path)?;

        println!("Variation image saved to {}", output_path);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tokio;

    const API_KEY: &str = "your_api_key_here";

    #[test]
    fn test_string_to_image() {
        let contents = fs::read_to_string("decode_example.txt").expect("Failed to read file");
        let _ = string_to_image(contents.as_str());
    }

    #[test]
    fn test_image_to_base64_string() {
        let input_image_path = "path/to/image.png";
        let input_image = image::open(input_image_path).expect("Failed to open image file");
        let _ = image_to_base64_string(&input_image);
    }

    #[tokio::test]
    async fn test_generate_image() {
        let prompt = "A beautiful sunset over the ocean";
        let output_prefix = "sunset in the universe";

        let result = generate_image(prompt, output_prefix, API_KEY, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_variations() {
        let input_path = "path/to/input/image.png";
        let output_prefix = "output";

        let result = generate_variations(input_path, output_prefix, API_KEY, None).await;
        assert!(result.is_ok());
    }
}
