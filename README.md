# 🦀 Rust Core Spike Prototype for zeitkapsl

Hey there! 👋

We’re [zeitkapsl.eu](https://zeitkapsl.eu/en/?utm_source=github_rust_intern), building a privacy-first photo backup and sharing app based in the EU, and we’re exploring whether we can **replace our current GoMobile-based core with Rust** to better support:

- 📱 Mobile (iOS + Android)
- 🖥️ Desktop
- 🌐 Web (via WASM)

This project is a **proof of concept** to validate if Rust is the right foundation for our core functionality — and you're going to help us figure that out!
Pick any or multiple topic(s) from the list below that sounds interesting for you:

## 🧱 Current Pain Points

We already have a shared core implemented in [Go](https://go.dev), exposed to mobile via [GoMobile](https://pkg.go.dev/golang.org/x/mobile/cmd/gomobile). While this setup works reasonably well, we’ve run into several persistent limitations:

- **💡 Poor type interoperability:**  
  GoMobile only supports a very limited set of types — no slices, arrays, maps, or complex/nested structs. We've had to rely on Protobuf as a workaround to pass structured data across the FFI boundary.

- **🧼 Type massaging:**  
  Even with Protobuf, we often need to "massage" or flatten types manually to make them FFI-compatible. This adds friction and complexity to evolving the core logic.

- **🛠️ Difficult cross-compilation:**  
  CGO dependencies (e.g., for FFmpeg, SQLite) complicate cross-platform builds. We've resorted to using [Zig](https://ziglang.org/) to patch over the pain, but it’s still brittle.

- **🌐 No shared core for Web:**  
  Our current Go setup doesn’t compile cleanly to WASM. Go WASM binaries are very large and [tinygo](https://tinygo.org/) would require a near rewrite. As a result, our Web client uses a completely separate implementation.

- **🕵️ Debugging across FFI is painful:**  
  Once you're across the boundary, debugging is basically guesswork. Breakpoints and stack traces become near-useless.

- **🚫 No native async/coroutine support:**  
  GoMobile doesn’t integrate well with Kotlin Coroutines or Swift `async/await`. We’ve had to implement workarounds using callback hell or custom threading models.

---

## 🔍 Possible Alternatives to Rust

Before fully committing to Rust, we’re open to comparing other approaches. Some potential alternatives include:

- [**Kotlin Multiplatform**](https://kotlinlang.org/docs/multiplatform.html)  
  Could be a good fit if we lean further into the Kotlin ecosystem. It offers solid sharing between Android, iOS, and Web — though support for low-level processing (e.g., image/video/crypto) is limited compared to Rust or C.

- [**Zig**](https://ziglang.org/)  
  An exciting language with first-class cross-compilation and a C-like FFI story. Promising, but still pre-1.0 and lacks the ecosystem maturity.

- [**C/C++ with manual bindings**]  
  We've also considered falling back to C/C++ for native interop, but to be honest I have done far too much C and C++ in my life.

- [**Your suggestion here!**]  
  We’re curious what other options you think might be viable — especially if you’ve worked on high-performance multiplatform apps.

---

## 🎯 Goal

Create a Rust-based core that supports critical features like:

- 🗃️ Local SQLite DB access
- 🖼️ Image + 📽️ Video processing
- 🔐 End-to-end encryption
- 🌍 HTTP API interactions
- 🔄 Cross-platform FFI (Android, iOS, Desktop)
- 🧠 WASM reusability for our web frontend

---

## 📦 What You'll Prototype

### 🧩 Database Layer (SQLite)
Implement CRUD access using Rust for these tables:

- `media`
- `collection`
- `collection_media`
- `search_label`
- `media_search_label`
- `settings(key,value)`

Maybe use `rusqlite` (or `sqlx` with SQLite backend)

---

### 🖼️ Image Pipeline

- Decode **HEIC**
- Resize using **Lanczos3**
- Encode to **WebP**
- Cache resized variants on disk

📏 Variants to generate:
- 1920px long edge (1920le)
- 250px height (250h)
- 250px square (250sq)

---

### 📽️ Video Processing

- Generate thumbnails from videos
- Transcode videos to efficient upload formats
- Parse MP4 headers
- Chunk and mux videos into **HLS segments**

Use **FFmpeg** via command-line and handle **streaming I/O** from Rust.

---

### 🔐 Encryption

- AES-GCM-256 encryption/decryption
- PBKDF2 and HKDF for key derivation
- SHA3 hashing
- Secure key generation using cryptographically secure RNG

Minimal dependencies (e.g. `ring`, `aes-gcm`, `sha3`, etc.)

---

### 🌐 HTTP Client

- Make HTTP(S) requests (GET, PUT, POST)
- Upload encrypted files via `PUT`
- Handle timeouts, retries

Use `reqwest` or `ureq`.

---

### 🧠 AI Model Inference

- Load and run ONNX models using `onnxruntime`
- Use for features like media labeling, object detection

Optional but valuable for evaluating on-device AI use.

---

### 🗂️ Filesystem Access

- Load media files
- Write to cache
- Manage temp files and directories

---

### 🧠 EXIF Handling

- Parse EXIF metadata (timestamp, GPS, orientation)
- Update EXIF tags after processing (e.g., orientation fix)

---

### 📦 Local File Cache

Implement a simple **LRU file cache** to manage thumbnails and processed assets.

---

### ⚙️ Parallel Processing

- Build a **configurable worker pool**
- Background jobs (resizing, encrypting, uploading) should run concurrently

Use Rust threads or `tokio`/`async-std`.

---

### 🔌 FFI & Platform Bindings

#### iOS
- Expose C ABI headers via `cbindgen`
- Interop with Swift using bridging headers

#### Android
- Expose JNI bindings via `jni` or `uniffi`
- Kotlin/Java integration

#### Desktop
- Pure Rust crate or C ABI (for embedding)

Make long-running functions **cancellable**, and explore **async Rust** where appropriate.

---

### 🌐 WASM Module

- Build a WASM version of:
  - EXIF reader
  - Image resizer
  - WebP encoder
- Use in our **browser-based client**
- Bonus: use it in webworkers

Deliver a working JS demo (ideally with Svelte) that uses the WASM module to process a local image.

---

### 🔁 Sample Workflow

```rust
fn import_file(path: &str) -> MediaObject;
fn process(media: &MediaObject) -> ProcessedMedia;
fn upload(media: &ProcessedMedia) -> UploadResult;
```

### Sample Project structure

```
zeitkapsl-rust-core/
├── Cargo.toml
├── src/
│   ├── ffi/                   # Platform bindings for iOS, Android, Desktop, Wasm
│   ├── db.rs                  # SQLite models & CRUD
│   ├── image.rs               # HEIC decoding, WebP encoding, resizing
│   ├── media.rs               # `MediaObject` logic
│   ├── upload.rs              # HTTP upload logic with encryption
│   ├── crypto.rs              # AES-GCM, PBKDF2, HKDF, SHA3
│   ├── exif.rs                # Read/write EXIF metadata
│   ├── ai.rs                  # ONNX model loading and inference
│   ├── ffmpeg.rs              # FFmpeg piping & HLS muxing
│   ├── cache.rs               # LRU filesystem cache
│   ├── worker.rs              # Configurable thread pool
│   ├── log.rs                 # Structured logging
│   └── server.rs              # Embedded local HTTP server
```


## 💌 Apply / Contribute

If you're a student or intern looking to work on real-world Rust across platforms, this is for you.

- 📫 jobs@zeitkapsl.eu
- 🐙 Or open an issue/PR in this repo
