use std::{io::Write, path::PathBuf};

const ESCAPE: &str = "\u{001b}[0m";

const BLACK: &str = "\u{001b}[40m";
const RED: &str = "\u{001b}[41m";
const GREEN: &str = "\u{001b}[42m";
const YELLOW: &str = "\u{001b}[43m";
const BLUE: &str = "\u{001b}[44m";
const MAGENTA: &str = "\u{001b}[45m";
const CYAN: &str = "\u{001b}[46m";
const WHITE: &str = "\u{001b}[47m";

pub enum ErrorString {
    Help,
    Warning,
    Error,
}

impl ErrorString {
    fn to_str(&self) -> String {
        match self {
            ErrorString::Help => format!("{}Error{}", RED, ESCAPE),
            ErrorString::Warning => format!("{}Warning{}", YELLOW, ESCAPE),
            ErrorString::Error => format!("{}Help{}", BLUE, ESCAPE),
        }
    }
}

pub enum ErrorLine {
    EmptyLine {
        line_number: u64,
        line_string: String,
    },
    SuffixArrowError {
        line_number: u64,
        line_string: String,
        error_msg: String,
    },
    InlineArrowError {
        line_number: u64,
        offset: u64,
        line_string: String,
        error_msg: String,
    },
}

impl ErrorLine {
    const LINE_COLOR: &str = CYAN;

    fn get_order(&self) -> u64 {
        match self {
            ErrorLine::EmptyLine { line_number, .. } => *line_number,
            ErrorLine::SuffixArrowError { line_number, .. } => *line_number,
            ErrorLine::InlineArrowError { line_number, .. } => *line_number,
        }
    }

    fn to_str(&self) -> String {
        match self {
            ErrorLine::EmptyLine {
                line_number,
                line_string,
            } => format!("{}{}\n", Self::get_line_bar(line_number), line_string),
            ErrorLine::SuffixArrowError {
                line_number,
                line_string,
                error_msg,
            } => format!(
                "{}{} {}<--{}{}\n",
                Self::get_line_bar(line_number),
                line_string,
                YELLOW,
                ESCAPE,
                error_msg
            ),
            ErrorLine::InlineArrowError {
                line_number,
                offset,
                line_string,
                error_msg,
            } => {
                let line_bar = Self::get_line_bar(line_number);
                let total_offset = line_bar.len() + *offset as usize;
                format!(
                    "{}{}\n{:width$}^ {}",
                    line_bar,
                    line_string,
                    error_msg,
                    width = total_offset as usize,
                )
            }
        }
    }

    fn get_line_bar(line_number: &u64) -> String {
        format!("{}|{} {}\t", Self::LINE_COLOR, ESCAPE, line_number)
    }
}

pub struct ErrorBuilder {
    header: Option<String>,
    footer: Option<String>,
    error_lines: Vec<ErrorLine>,
    id: Option<String>,
    file_path: Option<PathBuf>,
}

impl ErrorBuilder {
    pub fn new() -> Self {
        Self {
            header: None,
            footer: None,
            error_lines: Vec::new(),
            id: None,
            file_path: None,
        }
    }

    pub fn set_header(mut self, error_type: Option<ErrorString>, message: String) -> Self {
        self.header = match error_type {
            Some(error_type) => Some(format!("{}: {}\n", error_type.to_str(), message)),
            None => Some(message.to_string()),
        };
        println!("{}", self.header.clone().unwrap());
        self
    }
    pub fn set_footer(mut self, error_type: Option<ErrorString>, message: &str) -> Self {
        self.footer = match error_type {
            Some(error_type) => Some(format!("{}: {}\n", error_type.to_str(), message)),
            None => Some(message.to_string()),
        };
        self
    }

    pub fn set_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn set_file_path(mut self, file_path: PathBuf) -> Self {
        self.file_path = Some(file_path);
        self
    }

    pub fn add_error_line(mut self, error_line: ErrorLine) -> Self {
        self.error_lines.push(error_line);
        self
    }

    pub fn build(&mut self) -> String {
        let mut error_string = String::new();

        error_string = match &self.header {
            Some(header) => format!("{}{}", error_string, header),
            None => error_string,
        };
        println!("{}", self.header.take().unwrap());
        self.error_lines
            .sort_by_key(|error_line| error_line.get_order());
        for error_line in &self.error_lines {
            error_string = format!("{}{}", error_string, error_line.to_str());
        }

        error_string = match &self.footer {
            Some(footer) => format!("{}{}", error_string, footer),
            None => error_string,
        };
        error_string
    }
}
