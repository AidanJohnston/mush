use std::{
    io::{Error, Write},
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::engine::lexer::MushContext;

use super::output::{ErrorBuilder, ErrorLevel, ErrorLine};

static ERROR_TYPE_COUNTER: AtomicUsize = AtomicUsize::new(1);

/// Different types of errors found while parsing and "lexing" the mush script.
#[derive(Debug)]
#[repr(usize)]
pub enum MushError {
    /// When a character not in
    UnknownCharacter {
        unknown_character: char,
        ctx: MushContext,
    },
    /// When a string fails to contain a closing "
    IncompleteString { ctx: MushContext },
}

impl MushError {
    pub fn report(&self) -> String {
        match self {
            MushError::UnknownCharacter {
                unknown_character,
                ctx,
            } => ErrorBuilder::new()
                .set_header(
                    Some(ErrorLevel::Error),
                    self.id(),
                    format!("Unknown character '{}'.", unknown_character),
                )
                .set_file_path(ctx.file().to_path_buf())
                .add_error_line(ErrorLine::InlineSingleArrowError {
                    line_number: ctx.line_number(),
                    offset: ctx.offset(),
                    line_string: ctx.line().to_string(),
                    error_msg: "Unknown character.".to_string(),
                })
                .build(),
            MushError::IncompleteString { ctx } => todo!(),
        }
    }

    pub fn id(&self) -> String {
        match self {
            MushError::UnknownCharacter { .. } => Self::id_string(1),
            MushError::IncompleteString { .. } => Self::id_string(2),
        }
    }

    fn id_string(id: usize) -> String {
        format!("E{:04}", id)
    }
}
