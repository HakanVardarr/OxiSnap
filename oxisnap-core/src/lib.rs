pub mod capturer;
pub mod error;
pub mod image;
pub mod platforms;

pub use capturer::ScreenCapturer;
pub use error::{CaptureError, FfiError};

#[cfg(target_os = "macos")]
pub use platforms::NativeCapturer;
