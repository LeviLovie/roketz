# Roketz

A 2D battle game built with Rust, featuring terrain-based combat, physics, and collision detection.

## Getting Started

### Prerequisites

Make sure u have [Rust](https://www.rust-lang.org/learn/get-started), [FMOD Studio](https://www.fmod.com/download#fmodstudio) installed

Install FMOD Engine for MacOS and Linux installed at `fmod_bin/macos` and `fmod_bin/linux` respectively from [FMOD Engine](https://www.fmod.com/download#fmodengine).

### Run

```
just run
```

Alternatively you can enable the `fmod` feature on the `roketz` crate: `cargo run -p roketz --features fmod`

### Distribute

Run `just dist`, but i dont guarantee anything.

## License

MIT OR Apache-2.0 (see LICENSE-MIT and LICENSE-APACHE)
