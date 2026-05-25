<div align="center">

  <img src="docs/assets/logo.svg" alt="OxiSnap Logo" width="100" height="100" />

  <h1>OxiSnap <sup style="font-size: 0.4em; color: #888;">v0.1.0-alpha</sup></h1>

  <p>
    A <strong>blazing-fast, memory-safe</strong> screenshot capture and sharing tool built entirely in Rust — <br>
    hooking directly into native OS APIs with zero overhead and absolute safety guarantees.
  </p>

  <p>
    <img src="https://img.shields.io/badge/Rust-stable-orange?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Swift_5.9+-F54A2A?style=flat-square&logo=swift&logoColor=white" alt="Swift" />
    <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="MIT License" />
  </p>

</div>

<br>

---

## Overview

**OxiSnap** is a high-performance screenshot capture and sharing tool built entirely in Rust. Press a global hotkey from any app, and your screenshot is instantly captured, encrypted, stored, and a shareable link is copied to your clipboard — all in one step.

Capture uses Apple's [`ScreenCaptureKit`](https://developer.apple.com/documentation/screencapturekit) to extract raw pixel buffers straight from the GPU. Storage uses AES-256-GCM encryption before writing to SQLite, so images are always encrypted at rest.

> **Architecture note:** All OS-specific `unsafe` code is strictly encapsulated behind the `ScreenCapturer` trait inside `oxisnap-core`. Platform contributions must follow this contract.

---

## Core Technologies

| Crate / Library | Role |
|---|---|
| **Rust** (stable) | Backbone — memory safety without a GC, robust workspace architecture |
| **swift-rs** | Zero-cost Swift–Rust FFI bridge; handles ARC automatically, prevents leaks |
| **ScreenCaptureKit** | Apple's modern API for high-performance display stream capture |
| **image** (crate) | Raw-byte-to-PNG encoding and BGRA → RGBA channel swapping |
| **axum + tokio** | Async HTTP backend for image upload and retrieval |
| **sqlx + SQLite** | Async database — stores encrypted image blobs keyed by UUID |
| **aes-gcm** | AES-256-GCM authenticated encryption for images at rest |
| **rdev** | Global hotkey listener via `CGEventTap` |
| **arboard** | Cross-platform clipboard access |

---

## Workspace Structure

```
oxisnap/
├── oxisnap-core/      # The engine — OS-specific FFI and safe Rust traits
│   ├── src/
│   │   ├── capturer.rs          # ScreenCapturer trait
│   │   ├── image.rs             # BGRA → RGBA conversion, PNG buffer building
│   │   ├── error.rs             # CaptureError, FfiError
│   │   └── platforms/macos/
│   │       ├── mod.rs           # MacCapturer + ScreenCapturer impl
│   │       └── ffi.rs           # unsafe CGEventTap Swift bridge declarations
│   └── swift-lib/               # Swift package — ScreenCaptureKit integration
├── oxisnap-cli/       # Global hotkey daemon — capture, upload, clipboard
└── oxisnap-backend/   # HTTP server — encrypt, store, and serve screenshots
    ├── src/
    │   ├── crypto.rs            # AES-256-GCM encrypt / decrypt
    │   ├── db.rs                # SQLite setup, store, fetch
    │   ├── routes.rs            # POST /upload, GET /image/{id}
    │   └── error.rs             # BackendError + IntoResponse
    └── oxisnap.db               # SQLite database (created on first run)
```

---

## How It Works

```
Press Ctrl+Shift+S (any app)
  └─ oxisnap-cli detects hotkey via rdev CGEventTap
       └─ ScreenCaptureKit captures display → raw BGRA pixels
            └─ Rust swaps channels (BGRA → RGBA) → encodes PNG
                 └─ POST /upload → oxisnap-backend
                      └─ AES-256-GCM encrypt with random nonce
                           └─ Store ciphertext + nonce in SQLite (UUID key)
                                └─ Return http://host/image/<uuid>
                                     └─ Link copied to clipboard via arboard
```

When a client opens the link:
```
GET /image/<uuid>
  └─ Fetch encrypted blob + nonce from SQLite
       └─ AES-256-GCM decrypt using persisted key
            └─ Serve raw PNG bytes (Content-Type: image/png)
```

---

## Getting Started

### Prerequisites

- Rust toolchain (`stable`)
- Xcode Command Line Tools + Swift 5.9+
- macOS 14+ (Sonoma) — required by `SCScreenshotManager`

### Build from Source

```bash
git clone https://github.com/yourusername/oxisnap.git
cd oxisnap
cargo build --release
```

### Running

**Step 1 — Start the backend** (creates `oxisnap.db` and `oxisnap.key` on first run):

```bash
cargo run -p oxisnap-backend --release
```

**Step 2 — Start the CLI daemon** in a separate terminal:

```bash
cargo run -p oxisnap-cli --release
```

```
OxiSnap — press Ctrl+Shift+S to capture and share
```

Press `Ctrl+Shift+S` from any application. The CLI prints `Copied: http://...` and the link is on your clipboard.

### macOS Permissions

OxiSnap requires two permissions, both granted via **System Settings → Privacy & Security**:

| Permission | Required by | Why |
|---|---|---|
| **Screen Recording** | `oxisnap-core` | ScreenCaptureKit needs it to read display pixels |
| **Accessibility** | `oxisnap-cli` | `rdev`'s CGEventTap needs it to intercept global keypresses |

macOS will prompt for each on first use. Restart the relevant process after granting.

### Configuration

All options are set via environment variables:

| Variable | Default | Description |
|---|---|---|
| `PORT` | `3000` | Backend listen port |
| `OXISNAP_BASE_URL` | `http://localhost:<PORT>` | Base URL used in generated share links |
| `DATABASE_URL` | `sqlite:oxisnap.db?mode=rwc` | SQLite database path |
| `OXISNAP_BACKEND` | `http://127.0.0.1:3000/upload` | Upload endpoint the CLI posts to |

---

## Security

- Images are encrypted with **AES-256-GCM** before being written to the database. Each image gets a unique random 96-bit nonce.
- The encryption key is generated once on first startup and persisted to `oxisnap.key`. Guard this file — losing it makes stored images permanently unreadable.
- Links are UUID v4 — 122 bits of randomness. There is no index or listing endpoint.

---

## Roadmap

### ✅ Phase 1 — The Core Engine *(completed)*

- [x] Cargo Workspace architecture
- [x] macOS `ScreenCaptureKit` native integration
- [x] Memory-safe Swift–Rust FFI bridge via `swift-rs`
- [x] Async task isolation with synchronous C-ABI bridging
- [x] Automated padding/stride alignment and BGRA → RGBA channel swapping

### ✅ Phase 4 — Network & Sharing *(completed)*

- [x] Lightweight HTTP backend (axum + tokio)
- [x] UUID-keyed image storage in SQLite
- [x] AES-256-GCM encryption for all stored images
- [x] Instant shareable link generation
- [x] Global hotkey daemon (`Ctrl+Shift+S`) with automatic clipboard copy

### ⏸ Phase 2 — Platform Expansion *(deferred)*

- [ ] Windows — `Windows.Graphics.Capture`
- [ ] Linux — Wayland / PipeWire
- [ ] Multi-monitor detection and specific display targeting

### 🔜 Phase 3 — Image Processing & UX

- [ ] Interactive area selection (cropping)
- [ ] Configurable output formats: PNG, JPEG, WebP
- [ ] macOS native notifications on capture

### 🔜 Phase 5 — Hardening

- [ ] Configurable hotkey (currently fixed to `Ctrl+Shift+S`)
- [ ] Image expiry / deletion endpoint
- [ ] TLS support for the backend
- [ ] Peer-to-peer direct transfer (no server required)

---

## Contributing

Contributions, issues, and feature requests are welcome. A few ground rules:

- All OS-specific `unsafe` code must live inside `oxisnap-core`, behind the `ScreenCapturer` trait.
- New platform backends must not introduce coupling to `oxisnap-cli` or `oxisnap-backend`.
- Open an issue before submitting large PRs.

---

## License

Developed by **Hakan Vardar** — licensed under the [MIT License](LICENSE).
