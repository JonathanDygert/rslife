//! A library for simulating [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

#![warn(clippy::all, clippy::cargo_common_metadata)]
#![deny(
    future_incompatible,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    clippy::wildcard_dependencies
)]
#![forbid(unsafe_code)]

mod game;
mod opt;
mod point;

pub use game::Game;
pub use opt::Opt;
pub use point::Point;

pub mod gui;

/// The result type for the crate
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
