#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![warn(clippy::mut_mut)]
#![warn(clippy::iter_nth)]

mod engine;

pub use engine::SoundEngine;

pub mod bindings {
    include!("codegen/bindings.rs");
}
