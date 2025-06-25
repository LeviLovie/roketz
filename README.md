# Roketz

A 2D battle game built with Rust, featuring terrain-based combat with physics and collision detection.

## Features

- **2D Game Engine**: Built on Macroquad with egui for UI
- **Physics System**: Custom BVH (Bounding Volume Hierarchy) for efficient collision detection
- **Asset Pipeline**: Compile-time asset compilation for optimal performance
- **Scene Management**: Modular scene system for different game states
- **Debug Tools**: Comprehensive debugging interface with overlays
- **Configuration**: Flexible config system with hot-reload support

## Prerequisites

- Rust 1.70+ (2024 edition)
- Cargo
- Git

## Installation

1. Clone the repository:
```bash
git clone https://github.com/LeviLovie/roketz.git
cd roketz
```

2. Build the project:
```bash
cargo build --release
```

3. Run the game:
```bash
cargo run --release
```

## Development Setup

1. Install pre-commit hooks:
```bash
pre-commit install
```

2. Run tests:
```bash
cargo test
```

3. Run benchmarks:
```bash
cargo bench
```

4. Check code quality:
```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo deny check
```

5. Install development tools (optional):
```bash
cargo install cargo-deny
cargo install cargo-flamegraph
cargo install cargo-watch
```

## Development Commands

### Build Commands
- `cargo build --release` - Build release version
- `cargo build` - Build debug version
- `cargo clean` - Clean build artifacts

### Testing Commands
- `cargo test` - Run all tests
- `cargo bench` - Run benchmarks

### Code Quality
- `cargo fmt` - Format code
- `cargo fmt --check` - Check formatting
- `cargo clippy -- -D warnings` - Run clippy linter
- `cargo deny check` - Run cargo-deny

### Development Workflow
- `cargo run` - Run in debug mode
- `cargo run --release` - Run in release mode
- `cargo watch -x check -x test -x run` - Watch mode for development (requires cargo-watch)

### Documentation
- `cargo doc --no-deps --open` - Generate and open documentation

### Performance
- `cargo flamegraph --bin roketz` - Generate performance flamegraph (requires cargo-flamegraph)

## Controls

- **F3**: Toggle debug interface
- **Debug Menu**: Access performance metrics, overlays, and system controls

## Asset Development

Assets are defined in `assets/assets.ron` and compiled at build time. See the assets directory for examples.

## Configuration

The game creates a config file at `~/.config/roketz/config.ron` on first run. You can modify:
- Window dimensions
- Graphics scaling
- Physics parameters
- Collision detection settings

## Architecture

- **`src/game/`**: Main game loop and scene management
- **`src/ecs/`**: Entity Component System (Bevy ECS integration)
- **`src/bvh/`**: Bounding Volume Hierarchy for collision detection
- **`src/scenes/`**: Different game scenes and states
- **`assets_crate/`**: Asset compilation and management

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run all tests and checks before submitting:
```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo bench
cargo deny check
```
5. Submit a pull request

## License

MIT OR Apache-2.0 - see LICENSE file for details.