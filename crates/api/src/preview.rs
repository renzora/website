use crate::error::ApiError;
use uuid::Uuid;

/// Generate a watermarked image preview.
///
/// Resizes to max 800px on the longest side and overlays a semi-transparent
/// diagonal stripe pattern with "PREVIEW" rendered as pixel blocks.
/// Returns JPEG bytes.
pub fn generate_image_preview(data: &[u8]) -> Result<Vec<u8>, ApiError> {
    use image::{DynamicImage, Rgba, ImageFormat};

    let img = image::load_from_memory(data)
        .map_err(|e| ApiError::Internal(format!("Failed to decode image for preview: {e}")))?;

    // Resize to max 800px on longest side
    let resized = img.resize(800, 800, image::imageops::FilterType::Lanczos3);
    let mut rgba = resized.to_rgba8();

    let (w, h) = (rgba.width(), rgba.height());

    // Draw diagonal semi-transparent stripes
    draw_diagonal_stripes(&mut rgba, w, h);

    // Stamp "PREVIEW" text as pixel blocks tiled across the image
    let block_size = (w.min(h) / 80).max(2);
    stamp_preview_text(&mut rgba, w, h, block_size);

    let result = DynamicImage::ImageRgba8(rgba);
    let mut buf = Vec::new();
    result
        .write_to(&mut std::io::Cursor::new(&mut buf), ImageFormat::Jpeg)
        .map_err(|e| ApiError::Internal(format!("Failed to encode preview image: {e}")))?;

    Ok(buf)
}

/// Draw diagonal semi-transparent stripes across the image.
fn draw_diagonal_stripes(img: &mut RgbaImage, w: u32, h: u32) {
    use image::Rgba;
    let stripe_width = (w.min(h) / 12).max(8);
    let gap = stripe_width * 3;

    for y in 0..h {
        for x in 0..w {
            let diag = (x + y) % (stripe_width + gap);
            if diag < stripe_width {
                let pixel = img.get_pixel(x, y);
                let alpha_blend = 0.85_f32; // keep 85% of original
                let overlay = 255_u8; // white stripe
                let r = (pixel[0] as f32 * alpha_blend + overlay as f32 * (1.0 - alpha_blend)) as u8;
                let g = (pixel[1] as f32 * alpha_blend + overlay as f32 * (1.0 - alpha_blend)) as u8;
                let b = (pixel[2] as f32 * alpha_blend + overlay as f32 * (1.0 - alpha_blend)) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
            }
        }
    }
}

use image::RgbaImage;

// 5x7 pixel font for "PREVIEW" — each letter is a column of 7 bits, 5 columns wide
const LETTER_P: [[u8; 5]; 7] = [
    [1,1,1,1,0],[1,0,0,0,1],[1,0,0,0,1],[1,1,1,1,0],[1,0,0,0,0],[1,0,0,0,0],[1,0,0,0,0],
];
const LETTER_R: [[u8; 5]; 7] = [
    [1,1,1,1,0],[1,0,0,0,1],[1,0,0,0,1],[1,1,1,1,0],[1,0,1,0,0],[1,0,0,1,0],[1,0,0,0,1],
];
const LETTER_E: [[u8; 5]; 7] = [
    [1,1,1,1,1],[1,0,0,0,0],[1,0,0,0,0],[1,1,1,0,0],[1,0,0,0,0],[1,0,0,0,0],[1,1,1,1,1],
];
const LETTER_V: [[u8; 5]; 7] = [
    [1,0,0,0,1],[1,0,0,0,1],[1,0,0,0,1],[0,1,0,1,0],[0,1,0,1,0],[0,0,1,0,0],[0,0,1,0,0],
];
const LETTER_I: [[u8; 5]; 7] = [
    [1,1,1,1,1],[0,0,1,0,0],[0,0,1,0,0],[0,0,1,0,0],[0,0,1,0,0],[0,0,1,0,0],[1,1,1,1,1],
];
const LETTER_W: [[u8; 5]; 7] = [
    [1,0,0,0,1],[1,0,0,0,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,1,0,1],[1,1,0,1,1],[1,0,0,0,1],
];

const PREVIEW_LETTERS: [&[[u8; 5]; 7]; 7] = [
    &LETTER_P, &LETTER_R, &LETTER_E, &LETTER_V, &LETTER_I, &LETTER_E, &LETTER_W,
];

/// Stamp "PREVIEW" as pixel-block text tiled across the image.
fn stamp_preview_text(img: &mut RgbaImage, w: u32, h: u32, block: u32) {
    use image::Rgba;

    // Each letter is 5 blocks wide + 1 block gap = 6 blocks per letter
    // "PREVIEW" is 7 letters = 42 blocks wide + padding
    let text_w = 7 * 6 * block;
    let text_h = 7 * block;
    let step_x = (text_w as f32 * 1.8) as u32;
    let step_y = (text_h as f32 * 3.5) as u32;

    let color = Rgba([255u8, 255, 255, 60]);

    let mut row = 0u32;
    let mut base_y = 0i32 - (step_y as i32 / 2);
    while base_y < h as i32 + step_y as i32 {
        let offset = if row % 2 == 0 { 0 } else { step_x as i32 / 2 };
        let mut base_x = 0i32 - (step_x as i32 / 2) + offset;
        while base_x < w as i32 + step_x as i32 {
            // Draw each letter
            for (li, letter) in PREVIEW_LETTERS.iter().enumerate() {
                let lx = base_x + (li as i32 * 6 * block as i32);
                for (cy, row_data) in letter.iter().enumerate() {
                    for (cx, &on) in row_data.iter().enumerate() {
                        if on == 1 {
                            // Draw a block of `block x block` pixels
                            for by in 0..block {
                                for bx in 0..block {
                                    let px = lx + cx as i32 * block as i32 + bx as i32;
                                    let py = base_y + cy as i32 * block as i32 + by as i32;
                                    if px >= 0 && px < w as i32 && py >= 0 && py < h as i32 {
                                        let existing = img.get_pixel(px as u32, py as u32);
                                        let a = color[3] as f32 / 255.0;
                                        let r = (existing[0] as f32 * (1.0 - a) + color[0] as f32 * a) as u8;
                                        let g = (existing[1] as f32 * (1.0 - a) + color[1] as f32 * a) as u8;
                                        let b = (existing[2] as f32 * (1.0 - a) + color[2] as f32 * a) as u8;
                                        img.put_pixel(px as u32, py as u32, Rgba([r, g, b, existing[3]]));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            base_x += step_x as i32;
        }
        base_y += step_y as i32;
        row += 1;
    }
}

/// Generate a 30-second audio preview with periodic beep watermark using ffmpeg.
///
/// Takes the raw audio bytes and file extension, returns MP3 bytes of the preview.
pub async fn generate_audio_preview(data: &[u8], ext: &str) -> Result<Vec<u8>, ApiError> {
    let id = Uuid::new_v4();
    let input_path = std::env::temp_dir().join(format!("preview_in_{id}.{ext}"));
    let output_path = std::env::temp_dir().join(format!("preview_out_{id}.mp3"));

    tokio::fs::write(&input_path, data)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to write temp audio: {e}")))?;

    let output = tokio::process::Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            input_path.to_str().unwrap_or("input"),
            "-t",
            "30",
            "-filter_complex",
            "sine=frequency=1000:duration=0.2:sample_rate=44100,aloop=loop=-1:size=220500,atrim=duration=30[beep];[0:a]atrim=duration=30[main];[main][beep]amix=inputs=2:duration=shortest:weights=1 0.15[out]",
            "-map",
            "[out]",
            "-ar",
            "44100",
            "-b:a",
            "128k",
            output_path.to_str().unwrap_or("output"),
        ])
        .output()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to run ffmpeg: {e}")))?;

    let _ = tokio::fs::remove_file(&input_path).await;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let _ = tokio::fs::remove_file(&output_path).await;
        tracing::warn!("ffmpeg preview generation failed: {stderr}");
        return Err(ApiError::Internal("Audio preview generation failed".into()));
    }

    let result = tokio::fs::read(&output_path)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to read preview output: {e}")))?;

    let _ = tokio::fs::remove_file(&output_path).await;
    Ok(result)
}

/// Check if a MIME type supports preview generation.
pub fn is_previewable(mime: &str) -> bool {
    mime.starts_with("image/") || mime.starts_with("audio/")
}
