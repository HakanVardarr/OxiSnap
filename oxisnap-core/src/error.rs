use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaptureError {
    #[error("Screen capture failed (Check OS permissions or FFI API")]
    CaptureFailed,
    #[error("Failed to create ImageBuffer from raw pixel data.")]
    BufferCreationError,
}
