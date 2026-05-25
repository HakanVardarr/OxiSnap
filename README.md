<div align="center">

  <img src="docs/assets/logo.svg" alt="OxiSnap Logo" width="100" height="100" />

  <h1>OxiSnap <sup style="font-size: 0.4em; color: #888;">v0.1.0-alpha</sup></h1>

  <p>
    A <strong>blazing-fast, memory-safe</strong> screenshot extraction and sharing tool built entirely in Rust — <br>
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

**OxiSnap** is a high-performance screenshot extraction and sharing tool built entirely in Rust. By hooking directly into Apple's [`ScreenCaptureKit`](https://developer.apple.com/documentation/screencapturekit), it extracts raw pixel buffers straight from the GPU — bypassing traditional screenshot utilities entirely.

The result: zero-copy capture, zero latency, and absolute memory safety guaranteed by the Rust compiler.

> **Architecture note:** All OS-specific `unsafe` code is strictly encapsulated behind the `ScreenCapturer` trait inside `capturer-core`. Platform contributions must follow this contract.

---

## Core Technologies

| Crate / Library | Role |
|---|---|
| **Rust** (stable) | Backbone — memory safety without a GC, robust workspace architecture |
| **swift-rs** | Zero-cost Swift–Rust FFI bridge; handles ARC automatically, prevents leaks |
| **ScreenCaptureKit** | Apple's modern C/Swift API for high-performance display stream capture |
| **image** (crate) | Fast raw-byte-to-PNG encoding and BGRA → RGBA channel swapping |

---

## Workspace Structure

```
oxisnap/
├── capturer-core/    # The engine — OS-specific FFI and safe Rust traits
├── capturer-cli/     # The frontend — lightweight terminal capture interface
└── backend/          # The network layer — future P2P and HTTP sharing logic
```

Each crate has a single responsibility and zero coupling to the others, making cross-platform additions straightforward.

---

## Roadmap

### ✅ Phase 1 — The Core Engine *(completed)*

- [x] Cargo Workspace architecture
- [x] macOS `ScreenCaptureKit` native integration
- [x] Memory-safe Swift–Rust FFI bridge via `swift-rs`
- [x] Async task isolation with synchronous C-ABI bridging
- [x] Automated padding/stride alignment and BGRA → RGBA channel swapping

### ⏸ Phase 2 — Platform Expansion *(deferred)*

- [ ] Windows — `Windows.Graphics.Capture`
- [ ] Linux — Wayland / PipeWire (targeting Fedora and similar)
- [ ] Multi-monitor detection and specific display targeting

### 🔜 Phase 3 — Image Processing & UX

- [ ] Interactive CLI prompts for area selection (cropping)
- [ ] Direct system clipboard integration — bypass disk write entirely
- [ ] Configurable output formats: PNG, JPEG, WebP

### 🚧 Phase 4 — Network & P2P Sharing *(current focus)*

- [ ] Lightweight HTTP backend
- [ ] Peer-to-peer direct image transfer
- [ ] End-to-end encryption for shared image payloads
- [ ] Instant shareable link generation

---

## Getting Started

### Prerequisites

- Rust toolchain (`stable`)
- Xcode Command Line Tools
- Swift 5.9+

### Build from Source

OxiSnap uses a custom `build.rs` that automatically detects your host OS, compiles the native Swift/C libraries, and links all required dynamic runtime paths (`RPATH`).

```bash
# Clone the repository
git clone https://github.com/yourusername/oxisnap.git
cd oxisnap

# Build and run the CLI
cargo run -p capturer-cli --release
```

---

## Contributing

Contributions, issues, and feature requests are welcome. A few ground rules:

- All OS-specific `unsafe` code must live inside `capturer-core`, behind the `ScreenCapturer` trait.
- New platform backends should not introduce any coupling to `capturer-cli` or `backend`.
- Open an issue before submitting large PRs — let's align on the design first.

---

## License

Developed by **Hakan Vardar** — licensed under the [MIT License](LICENSE).