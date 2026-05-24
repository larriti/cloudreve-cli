//! CAPTCHA handling utilities for CLI
//!
//! This module provides functions for displaying CAPTCHA images
//! directly in the terminal using viuer, which supports Kitty, iTerm2,
//! and Sixel graphics protocols.

use base64::Engine;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

/// CAPTCHA display result
#[derive(Debug)]
pub struct CaptchaDisplay {
    /// Path to saved image file (for fallback or manual viewing)
    pub image_path: PathBuf,
    /// Whether the image was displayed inline in the terminal
    pub inline_displayed: bool,
}

/// Display CAPTCHA in terminal or save to file as fallback
///
/// This function attempts to display the CAPTCHA image directly
/// in the terminal using viuer. viuer will automatically detect
/// and use the best supported graphics protocol (Kitty > iTerm2 > Sixel > ASCII).
///
/// # Arguments
///
/// * `image_data` - Base64 encoded image data (usually PNG with data URL prefix)
///
/// # Returns
///
/// A `CaptchaDisplay` struct containing the path to the saved image file
/// and whether inline display succeeded
pub fn display_captcha(image_data: &str) -> Result<CaptchaDisplay, String> {
    // Decode base64 image data
    let image_bytes = decode_base64_image(image_data)?;

    // Load the image
    let img = image::load_from_memory(&image_bytes)
        .map_err(|e| format!("Failed to load image: {}", e))?;

    // Configure viuer
    // Use a reasonable size for CAPTCHA display
    let config = viuer::Config {
        width: Some(60),
        height: Some(10),
        x: 0,
        y: 0,
        absolute_offset: false,
        ..Default::default()
    };

    // Try to display inline in terminal
    let inline_displayed = viuer::print(&img, &config).is_ok();

    // Always save to temp file as backup
    let temp_file = save_to_temp_file(&image_bytes)?;

    Ok(CaptchaDisplay {
        image_path: temp_file.path().to_path_buf(),
        inline_displayed,
    })
}

/// Decode base64 image data
fn decode_base64_image(image_data: &str) -> Result<Vec<u8>, String> {
    // Handle data URL format if present (e.g., "data:image/png;base64,...")
    let data = if image_data.starts_with("data:image/") {
        // Extract the base64 part after the comma
        image_data
            .split(',')
            .nth(1)
            .ok_or("Invalid data URL format")?
    } else {
        image_data
    };

    base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|e| format!("Failed to decode base64 image: {}", e))
}

/// Save image data to a temporary file
fn save_to_temp_file(image_data: &[u8]) -> Result<NamedTempFile, String> {
    let mut temp_file =
        NamedTempFile::new().map_err(|e| format!("Failed to create temp file: {}", e))?;

    temp_file
        .write_all(image_data)
        .map_err(|e| format!("Failed to write image data: {}", e))?;

    temp_file
        .flush()
        .map_err(|e| format!("Failed to flush temp file: {}", e))?;

    Ok(temp_file)
}

/// Print CAPTCHA prompt to user
///
/// Displays instructions to the user about how to view the CAPTCHA
pub fn print_captcha_prompt(display: &CaptchaDisplay) {
    use std::io::{Write, stdout};

    if display.inline_displayed {
        println!("\n[CAPTCHA displayed above]");
        println!(
            "If the image is not visible, the file is saved at: {}",
            display.image_path.display()
        );
        print!("Please enter the CAPTCHA code: ");
    } else {
        println!("\nCAPTCHA image saved to: {}", display.image_path.display());
        println!("Your terminal does not support inline image display.");
        print!("Please open the image file and enter the CAPTCHA code: ");
    }

    // Flush immediately to ensure prompt is visible
    let _ = stdout().flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_base64_image() {
        // A simple 1x1 red PNG in base64
        let base64_png = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==";
        let result = decode_base64_image(base64_png);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_decode_base64_image_with_data_url() {
        let data_url = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==";
        let result = decode_base64_image(data_url);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_base64_image_invalid() {
        let invalid_data = "not-valid-base64!!!";
        let result = decode_base64_image(invalid_data);
        assert!(result.is_err());
    }
}
