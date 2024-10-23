use std::io::{Error, Write};

use crate::engine::lexer::MushContext;

use super::output::{ErrorBuilder, ErrorLine, ErrorString};

pub enum MushError {
    UnknownCharacter {
        unknown_character: char,
        ctx: MushContext,
    },
    IncompleteString {
        ctx: MushContext,
    },
}

impl MushError {
    pub fn report(&self) -> String {
        match self {
            MushError::UnknownCharacter {
                unknown_character,
                ctx,
            } => ErrorBuilder::new()
                .set_header(
                    Some(ErrorString::Error),
                    format!("Unknown character '{}'.", unknown_character),
                )
                .set_file_path(ctx.file().to_path_buf())
                .add_error_line(ErrorLine::InlineArrowError {
                    line_number: ctx.line_number(),
                    offset: ctx.offset(),
                    line_string: ctx.line().to_string(),
                    error_msg: "Unknown character.".to_string(),
                })
                .build(),
            MushError::IncompleteString { ctx } => todo!(),
        }
    }
}
