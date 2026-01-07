use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    code: ErrorCode,
    note: String,
    meta: HashMap<String, String>,
}

impl Error {
    pub fn new(note: &str, code: ErrorCode) -> Self {
        Self {
            note: (if !note.is_empty() {
                note
            } else {
                match code {
                    ErrorCode::Tech => "A technical issue occured. Please try again later.",
                    ErrorCode::User => "An issue occured due to your input.",
                    ErrorCode::NotFound => "The information you seek was not found.",
                }
            })
            .to_string(),
            code,
            meta: HashMap::new(),
        }
    }

    pub fn user(note: &str) -> Self {
        Self::new(note, ErrorCode::User)
    }

    pub fn tech(note: &str) -> Self {
        Self::new(note, ErrorCode::Tech)
    }

    pub fn notfound(note: &str) -> Self {
        Self::new(note, ErrorCode::NotFound)
    }

    pub fn add_meta(&mut self, key: &str, val: &str) {
        self.meta.insert(key.to_string(), val.to_string());
    }

    pub fn add_meta_x(mut self, key: &str, val: &str) -> Self {
        self.meta.insert(key.to_string(), val.to_string());
        self
    }

    pub fn has_meta(&self) -> bool {
        self.meta.len() > 0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ErrorCode {
    Tech,
    User,
    NotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        let mut err = Error::tech("");
        err.add_meta("from", "varerror");
        err.add_meta("error", &e.to_string());
        err
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        let mut err = Error::tech("");
        err.add_meta("from", "parse_int_err");
        err.add_meta("error", &e.to_string());
        err
    }
}

// impl From<std::io::Error> for Error {
//     fn from(e: std::io::Error) -> Self {
//         let mut err = Error::tech("");
//         err.add_meta("from", "io_err");
//         err.add_meta("error", &e.to_string());
//         err
//     }
// }
