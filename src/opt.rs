//! Define and parse the CLI.

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Conway's Game of Life.
pub struct Opt {
    #[structopt(long = "speed", default_value = "30")]
    /// Updates per second
    pub speed: u64,
    #[structopt(subcommand)]
    /// Main command
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
/// Main command.
pub enum Command {
    #[structopt(name = "rle")]
    /// Reads a pattern from an rle file
    Rle {
        /// The path to the rle file
        file: PathBuf,
    },
    #[structopt(name = "random")]
    /// Creates a random board within given dimensions
    Random {
        /// The board width
        width: u32,
        /// The board height
        height: u32,
    },
}
