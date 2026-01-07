# ⚙️ VoxTech ⛰️

## Overview
`VoxTech` is an open-source project aiming to implement a safe and high-performance 3D sandbox simulation in pure Rust.

### Goal
Build a Minecraft-like 3D FPS/TPS sandbox action simulator featuring a continuous, automated terrain generation system.

### Requirements
* Implement in Rust, with `unsafe` usage kept to an absolute minimum as a rule
* Target Windows/Linux and ensure a baseline level of cross-platform support
* Use data structures mindful of SIMD, multithreading, and cache behavior
  * Leverage fixed-length arrays and recursive data structures to balance storage efficiency and performance
  * Implement a map structure based on an extended octree (64-ary tree), organized as Cell/Chunk/Sector/Region

## Links & Special Thanks

### 🪁 Window creation / OS event handling: [winit](https://github.com/rust-windowing/winit)

### ✒️ Graphics: [wgpu](https://github.com/gfx-rs/wgpu)

### 🖼️ Image processing: [image](https://github.com/image-rs/image)

### 🔊 Audio: [rodio](https://github.com/RustAudio/rodio)

### 📈 Linear algebra: [nalgebra](https://github.com/dimforge/nalgebra)

### 📝 Logging: [tracing](https://github.com/tokio-rs/tracing)

### 📃 Serialization / Deserialization: [serde](https://github.com/serde-rs/serde)

### ⛓️ Concurrency - locking: [parking_lot](https://github.com/Amanieu/parking_lot)

### 📞 Concurrency - channels: [crossbeam](https://github.com/crossbeam-rs/crossbeam)

### 📦 Hash table: [hashbrown](https://github.com/rust-lang/hashbrown)

### And many other OSS/crates
We deeply appreciate the contributions! 🥰
