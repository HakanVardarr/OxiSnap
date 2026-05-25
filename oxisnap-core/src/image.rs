use ::image::{ImageBuffer, RgbaImage};

use crate::error::CaptureError;

pub fn bgra_to_rgba(pixels: &mut [u8]) {
    for chunk in pixels.chunks_exact_mut(4) {
        chunk.swap(0, 2);
    }
}

pub fn into_rgba_image(width: u32, height: u32, pixels: Vec<u8>) -> Result<RgbaImage, CaptureError> {
    ImageBuffer::from_raw(width, height, pixels).ok_or_else(|| CaptureError::PixelBuffer {
        reason: format!("buffer size does not match {}x{}", width, height),
    })
}
