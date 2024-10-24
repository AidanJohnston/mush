use std::{io::Write, path::PathBuf};

use super::error::MushError;

const ESCAPE: &str = "\u{001b}[0m";

const BOLD: &str = "\u{001b}[1m";

const BLACK: &str = "\u{001b}[30m";
const RED: &str = "\u{001b}[31m";
const GREEN: &str = "\u{001b}[32m";
const YELLOW: &str = "\u{001b}[33m";
const BLUE: &str = "\u{001b}[34m";
const MAGENTA: &str = "\u{001b}[35m";
const CYAN: &str = "\u{001b}[36m";
const WHITE: &str = "\u{001b}[37m";

pub enum ErrorLevel {
    Help,
    Warning,
    Error,
}

impl ErrorLevel {
    fn to_str(&self, id: String) -> String {
        match self {
            ErrorLevel::Help => format!("{}Help [{}]{}", BLUE, id, ESCAPE),
            ErrorLevel::Warning => format!("{}Warning [{}]{}", YELLOW, id, ESCAPE),
            ErrorLevel::Error => format!("{}Error [{}]{}", RED, id, ESCAPE),
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
    InlineSingleArrowError {
        line_number: u64,
        offset: u64,
        line_string: String,
        error_msg: String,
    },
}

impl ErrorLine {
    const LINE_COLOR: &str = CYAN;

    fn line_number(&self) -> u64 {
        match self {
            ErrorLine::EmptyLine { line_number, .. } => *line_number,
            ErrorLine::SuffixArrowError { line_number, .. } => *line_number,
            ErrorLine::InlineSingleArrowError { line_number, .. } => *line_number,
        }
    }

    fn to_str(&self, line_spacing: u64) -> String {
        match self {
            ErrorLine::EmptyLine {
                line_number,
                line_string,
            } => format!(
                "{}{}\n",
                Self::get_line_bar(line_number, &line_spacing),
                line_string
            ),
            ErrorLine::SuffixArrowError {
                line_number,
                line_string,
                error_msg,
            } => format!(
                "{}{} {}<--{}{}\n",
                Self::get_line_bar(line_number, &line_spacing),
                line_string,
                YELLOW,
                ESCAPE,
                error_msg
            ),
            ErrorLine::InlineSingleArrowError {
                line_number,
                offset,
                line_string,
                error_msg,
            } => {
                let line_bar = Self::get_line_bar(line_number, &line_spacing);
                format!("{}{}\n^ {}", line_bar, line_string, error_msg,)
            }
        }
    }

    fn get_line_bar(line_number: &u64, line_spacing: &u64) -> String {
        format!(
            "{}{:<width$}|\t{}",
            Self::LINE_COLOR,
            line_number,
            ESCAPE,
            width = *line_spacing as usize
        )
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

    pub fn set_header(
        mut self,
        error_level: Option<ErrorLevel>,
        id: String,
        message: String,
    ) -> Self {
        self.header = match error_level {
            Some(error_type) => Some(format!(
                "{}: {}{}{}\n",
                error_type.to_str(id),
                BOLD,
                message,
                ESCAPE
            )),
            None => Some(message.to_string()),
        };
        self
    }
    pub fn set_footer(mut self, error_type: Option<ErrorLevel>, id: String, message: &str) -> Self {
        self.footer = match error_type {
            Some(error_type) => Some(format!("{}: {}\n", error_type.to_str(id), message)),
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

        // HEADER
        error_string = match &self.header {
            Some(header) => format!("{}{}", error_string, header),
            None => error_string,
        };

        let line_spacing = self
            .error_lines
            .iter()
            .map(|error| error.line_number())
            .max()
            .unwrap_or(0)
            .max(4);

        // FILE PATH
        error_string = match &self.file_path {
            Some(path_buf) => format!(
                "{}{:<width$}{}-->{} {}\n",
                error_string,
                " ",
                BLUE,
                ESCAPE,
                path_buf.to_str().unwrap_or(""),
                width = line_spacing as usize - 1,
            ),
            None => error_string,
        };

        // ORDERED ERROR LINES
        self.error_lines
            .sort_by_key(|error_line| error_line.line_number());
        for error_line in &self.error_lines {
            error_string = format!("{}{}", error_string, error_line.to_str(line_spacing));
        }

        // FOOTER
        error_string = match &self.footer {
            Some(footer) => format!("{}{}", error_string, footer),
            None => error_string,
        };
        error_string
    }
}
