#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![warn(clippy::mut_mut)]
#![warn(clippy::iter_nth)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::needless_borrow)]

pub mod bvh;
pub mod camera;
pub mod config;
pub mod ecs;
pub mod game;
pub mod result;
pub mod scenes;
pub mod signals;
