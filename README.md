# Roketz

A 2D battle game built with Rust, featuring terrain-based combat, physics, and collision detection.

## Features
- 2D game engine using Macroquad and egui
- Custom physics and collision system
- Asset pipeline with compile-time asset bundling
- Modular scene management

## Getting Started

### Prerequisites
- Rust (2024 edition)
- Cargo

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

### Test
```bash
cargo test
```

## Project Structure
- `src/` - Main source code
- `assets/` - Game assets
- `assets_crate/` - Asset compilation logic
- `benches/` - Benchmarks

## License
MIT OR Apache-2.0 (see LICENSE)