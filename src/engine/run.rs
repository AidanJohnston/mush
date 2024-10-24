use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::diagnostics::error::MushError;
use super::file_extension::has_valid_file_extension;
use super::lexer::{MushContext, Scanner};

pub fn run_from_file(path: &Path) -> Result<(), std::io::Error> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    if !has_valid_file_extension(path) {
        let extension = match path.extension() {
            Some(ext) => ext.to_str().unwrap_or(""),
            None => "",
        };
        print!("Unknown file extension .{}", extension);
        return Ok(());
    }

    let buf_reader = BufReader::new(file);
    let mush_context = MushContext::new(path.to_path_buf(), "".to_string(), 0, 0);
    let mut scanner = Scanner::new(buf_reader, mush_context);
    scanner.scan_tokens()?;

    for error in scanner.errors() {
        println!("{}", error.report());
    }

    for token in scanner.tokens() {}
    Ok(())
}

pub fn run_iterupter() {}
