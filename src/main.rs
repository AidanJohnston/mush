use std::{
    env,
    io::{self, Error},
    path::Path,
};

use engine::run::{run_from_file, run_iterupter};

mod cli;
mod engine;

fn main() -> io::Result<()> {
    let matches = cli::get_cli_arg_matches();

    if let Some(value) = matches.get_one::<String>("file_path") {
        let _ = run_from_file(Path::new(value))?;
    } else {
        run_iterupter();
    }
    Ok(())
}
