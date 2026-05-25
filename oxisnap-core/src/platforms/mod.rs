#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "macos")]
pub type NativeCapturer = macos::MacCapturer;
