use clap::{Arg, Command};

pub fn get_cli_commnad() -> Command {
    return Command::new("mush")
        .version(env!("CARGO_PKG_VERSION"))
        .about("The mush scripting language.")
        .author("Aidan Johnston <contact@aidanjohnston.ca>")
        .arg(
            Arg::new("file_path")
                .index(1) // Specifies that this is a positional argument at position 1
                .help("Path to the mush scrip to run.")
                .required(false), // Makes it optional
        );
}
