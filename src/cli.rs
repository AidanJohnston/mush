use clap::{Arg, ArgMatches, Command};

pub fn get_cli_arg_matches() -> ArgMatches {
    return Command::new("mush")
        .arg(
            Arg::new("file_path")
                .index(1) // Specifies that this is a positional argument at position 1
                .help("Path to the mush scrip to run.")
                .required(false), // Makes it optional
        )
        .get_matches();
}
