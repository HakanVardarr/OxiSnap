# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
# Build all crates
cargo build

# Build release and run the CLI (captures a screenshot to screenshot.png)
cargo run -p oxisnap-cli --release

# Build a specific crate
cargo build -p oxisnap-core

# Lint check
cargo clippy -- -D warnings
```

No tests exist yet. The project currently has no lint configuration beyond `cargo check` / `cargo clippy`.

## Architecture

OxiSnap is a Cargo workspace with three crates:

- **`oxisnap-core`** — the engine. Defines the `ScreenCapturer` trait and platform implementations. On macOS, it bridges to Swift via `swift-rs` FFI. The Swift library (`swift-lib/`) is a separate Swift Package (`ScreenCaptureBridge`) compiled by `build.rs` at cargo build time using `SwiftLinker`. All `unsafe` and OS-specific code lives here, behind the `ScreenCapturer` trait.
- **`oxisnap-cli`** — thin CLI binary. Instantiates `NativeCapturer` (a type alias to the platform's capturer) and saves the result as `screenshot.png`.
- **`oxisnap-backend`** — placeholder for a future HTTP/P2P sharing layer. Currently empty.

### macOS FFI flow

```
cargo build
  └─ build.rs (oxisnap-core)
       └─ SwiftLinker compiles swift-lib/ into a static library
            └─ linked into oxisnap-core via rustflags in .cargo/config.toml (rpath to Swift runtime)

Runtime call:
  MacCapturer::capture()
    └─ unsafe { capture_screen_swift() }   ← swift-rs macro declares C-ABI symbol
         └─ Swift: SCScreenshotManager.captureImage(...)  (ScreenCaptureKit, macOS 14+)
              └─ returns raw BGRA pixels → Rust swaps channels to RGBA → image::RgbaImage
```

The Swift side uses a `DispatchSemaphore` to bridge the async `SCScreenshotManager` API into a synchronous C ABI call. Stride padding is stripped manually before the pixel buffer is handed to Rust.

### Adding a new platform

1. Add a new module under `oxisnap-core/src/platforms/`.
2. Implement the `ScreenCapturer` trait.
3. Add a `#[cfg(target_os = "...")]` branch in `build.rs`.
4. Expose `NativeCapturer` via `#[cfg]` in `lib.rs`.
5. Do not couple the new backend to `oxisnap-cli` or `oxisnap-backend`.

## Requirements

- Rust stable toolchain
- Xcode Command Line Tools + Swift 5.9+
- macOS 14+ (Sonoma) — `ScreenCaptureKit`'s `SCScreenshotManager` requires it
- Screen Recording permission must be granted to the terminal/binary in System Settings

---

## Error Handling

**Use `thiserror` for all library error types.** Never use `Box<dyn Error>` or `anyhow` inside `oxisnap-core` — those are for application-level code.

### Error type structure

Each crate/module defines its own error enum with `#[derive(Debug, thiserror::Error)]`. Errors bubble up through `From` impls that `thiserror` generates automatically via `#[from]`.

```
CaptureError (oxisnap-core)
  └─ FfiError          — FFI / Swift bridge failures
  └─ PixelBufferError  — pixel conversion failures
  └─ PermissionError   — OS permission denied

CliError (oxisnap-cli)
  └─ CaptureError (via #[from])
  └─ IoError           — file save failures
```

### Canonical pattern

```rust
// oxisnap-core/src/error.rs
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
```

```rust
// oxisnap-cli/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("capture failed: {0}")]
    Capture(#[from] oxisnap_core::CaptureError),

    #[error("failed to save image to {path}: {source}")]
    Io { path: String, #[source] source: std::io::Error },
}
```

### Rules

- **No `unwrap()` in library code** (`oxisnap-core`). Use `?` and propagate `Result`.
- **No `unwrap()` in `oxisnap-cli`** unless the invariant is guaranteed by construction and a `// SAFETY:` comment explains why.
- **No `panic!` in library code.** Panics are bugs, not error handling.
- **Every FFI boundary returns `Result`.** Null pointers, unexpected return codes, and timeouts are `FfiError` variants, not panics.
- **Every `unsafe` block must have a `// SAFETY:` comment** explaining the invariants that make it sound.
- **`thiserror` only in `oxisnap-core`.** `oxisnap-cli` may use `anyhow` for top-level `main` convenience, but its own error types still use `thiserror`.

---

## Modular Code Structure

### Module layout (oxisnap-core)

```
oxisnap-core/
├── src/
│   ├── lib.rs              — re-exports only; no logic
│   ├── error.rs            — CaptureError, FfiError, PixelBufferError
│   ├── capturer.rs         — ScreenCapturer trait definition
│   ├── image.rs            — pixel format conversion (BGRA → RGBA, stride stripping)
│   └── platforms/
│       ├── mod.rs          — NativeCapturer type alias + cfg guards
│       └── macos/
│           ├── mod.rs      — MacCapturer struct, ScreenCapturer impl
│           └── ffi.rs      — unsafe extern "C" declarations, raw FFI wrappers
```

### Responsibilities per file

| File | Allowed | Forbidden |
|---|---|---|
| `lib.rs` | `pub use`, `pub mod` | Any logic or types |
| `error.rs` | Error enums | Business logic |
| `capturer.rs` | `ScreenCapturer` trait | Platform-specific code |
| `image.rs` | Pure pixel math | I/O, FFI |
| `platforms/macos/ffi.rs` | `unsafe extern`, thin wrappers returning `Result` | Any logic beyond error conversion |
| `platforms/macos/mod.rs` | `ScreenCapturer` impl, calls `ffi.rs` | Direct `unsafe` blocks (delegate to `ffi.rs`) |

### Rules

- **`lib.rs` is a re-export file only.** No `fn`, no `struct`, no `impl` defined directly in it.
- **One concern per file.** `image.rs` is pure pixel conversion; it never calls FFI. `ffi.rs` is raw FFI only; it never interprets pixels.
- **Platform code never leaks upward.** `oxisnap-cli` and `oxisnap-backend` must not `use` anything from `platforms::*` directly — only through the `ScreenCapturer` trait and `NativeCapturer` alias.
- **`NativeCapturer` is the only public surface of the platform layer.** It is a type alias set by `#[cfg]` in `platforms/mod.rs`.
- **No business logic in `ffi.rs`.** It wraps raw calls and converts return values to `Result<_, FfiError>`. That is all.

---

## Coding Standards

- All `unsafe` blocks must have a `// SAFETY:` comment explaining the invariant.
- Prefer `?` over `match` for error propagation.
- No `clone()` calls unless genuinely required — document why with a comment.
- No `println!` in library code. Use `tracing` (or `log`) if observability is needed.
- Keep functions under ~40 lines. If a function is longer, split it.
- Public API items require doc comments (`///`).

---

## Do NOT

- Add logic to `oxisnap-cli` beyond argument parsing, wiring, and file I/O.
- Couple `oxisnap-backend` to any platform or FFI code.
- Call platform APIs directly from any crate other than `oxisnap-core`.
- Use `Box<dyn Error>` as a return type in library code.
- Use `unwrap()` or `expect()` in library code without a `// SAFETY:` comment and a compelling argument.
- Add `cargo clean`-invisible Swift changes — run `cargo clean && cargo build` after any Swift source edit.

---

## Common Gotchas

- If the build fails with linker errors, check `.cargo/config.toml` rpath settings first.
- Screen Recording permission must be re-granted after the binary path changes (e.g. after `cargo clean`).
- `cargo clean` is required after Swift source changes — `build.rs` does not always detect incremental Swift changes.
- `SCScreenshotManager` requires macOS 14+; building on 13.x will fail at runtime, not compile time.