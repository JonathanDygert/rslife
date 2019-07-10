//! Define and parse the CLI.

#![allow(missing_docs)]

use docopt::Docopt;
use serde::Deserialize;

const USAGE: &str = "
Conway's Game of Life.

Usage:
    rslife rle <file> [options]
    rslife random <width> <height> [options]
    rslife (-h | --help)

Options:
    -h --help  Show this screen.
    --speed=S  Updates per second [default: 30].

Controls:
    drag            Move view
    scroll up/down  Zoom in/out
    t               Toggle light/dark theme

Note:
    The framerate is locked at 60 fps. At update speeds higher than this,
    patterns may appear to be still or move strangely. The underlying
    simulation will still update correctly.
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub cmd_rle: bool,
    pub arg_file: String,

    pub cmd_random: bool,
    pub arg_width: u32,
    pub arg_height: u32,

    pub flag_speed: u64,
}

impl Args {
    pub fn parse() -> Self {
        Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit())
    }
}
