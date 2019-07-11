use structopt::StructOpt;

use rslife::{gui, Opt, Result};

fn main() -> Result<()> {
    gui::main(&Opt::from_args())
}
