use std::{
    io::Write,
    sync::atomic::{AtomicUsize, Ordering},
};

static ERROR_TYPE_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub enum MushError {
    UnknownCharacter(UnknownCharacter),
    IncompleteString(IncompleteString),
}

impl MushError {
    fn id(&self) -> String {
        let type_id: usize = ERROR_TYPE_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("E{:04}", type_id).to_owned()
    }
}

pub struct UnknownCharacter {
    character: char,
    line: i32,
    offset: i32,
}

impl UnknownCharacter {
    pub fn new(character: char, line: i32, offset: i32) -> Self {
        Self {
            character,
            line,
            offset,
        }
    }
}

pub struct IncompleteString {
    string: String,
    line: i32,
    offset: i32,
}
impl IncompleteString {
    pub fn new(string: String, line: i32, offset: i32) -> Self {
        Self {
            string,
            line,
            offset,
        }
    }
}
