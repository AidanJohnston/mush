use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::file_extension::has_valid_file_extension;
use super::lexer::Scanner;

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
    let mut scanner = Scanner::new(buf_reader);
    scanner.scan_tokens()?;
    for token in scanner.tokens() {
        println!("{}", token)
    }
    Ok(())
}

pub fn run_iterupter() {}
