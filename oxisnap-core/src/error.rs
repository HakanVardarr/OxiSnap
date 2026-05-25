use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaptureError {
    #[error("FFI call failed: {0}")]
    Ffi(#[from] FfiError),

    #[error("pixel buffer is malformed: {reason}")]
    PixelBuffer { reason: String },

    #[error("screen recording permission denied")]
    PermissionDenied,
}

#[derive(Debug, Error)]
pub enum FfiError {
    #[error("Swift returned a null pointer")]
    NullPointer,

    #[error("Swift capture timed out after {ms}ms")]
    Timeout { ms: u64 },
}
