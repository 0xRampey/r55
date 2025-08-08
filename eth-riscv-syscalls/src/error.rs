use alloc::borrow::Cow;
use core::{fmt, error::Error as CoreError};

/// Error related to syscall convertions
#[derive(Debug)]
pub enum Error {
    UnknownOpcode(u8),
    ParseError { input: Cow<'static, str> },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownOpcode(op) => write!(f, "Unknown syscall opcode: {op}"),
            Error::ParseError { input } => write!(f, "Parse error for syscall string. Input: {input}"),
        }
    }
}

impl CoreError for Error {}
