use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    match target_os.as_str() {
        "macos" => build_macos(),
        _ => {
            println!("cargo:warning=Unsupported operating system: {}", target_os);
        }
    }
}

fn build_macos() {
    use swift_rs::SwiftLinker;

    println!("cargo:warning=Building Swift integration for macOS...");

    SwiftLinker::new("14.0")
        .with_package("ScreenCaptureBridge", "./swift-lib")
        .link();
}
