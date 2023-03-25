use dotenv::dotenv;
use karlo_rs;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set in .env file");

    // Generate images based on prompt text
    let text = "A sunset in the universe";
    let output_name = "sample_img/output"; // will be png
    let batch_size = Some(2); // or Some(value) where value is between 1 and 8

    match karlo_rs::generate_image(text, output_name, &api_key, batch_size).await {
        Ok(_) => println!("Image saved to {}", output_name),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Generate images based on input image.
    let input_path = "sample_img/output_1.png";
    let output_name = "sample_img/output_variation"; // will be png
    let batch_size = None; // or Some(value) where value is between 1 and 8

    match karlo_rs::generate_variations(input_path, output_name, &api_key, batch_size).await {
        Ok(_) => println!("Variation image saved to {}", output_name),
        Err(e) => eprintln!("Error: {}", e),
    }
}
