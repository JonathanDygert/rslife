//! A library for simulating [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

#![deny(
    rust_2018_idioms,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![warn(clippy::all, clippy::cargo_common_metadata)]
#![deny(clippy::wildcard_dependencies)]

mod args;
mod game;
mod point;

pub use args::Args;
pub use game::Game;
pub use point::Point;

pub mod gui;

/// The result type for the crate
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
