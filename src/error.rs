use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    BadChunkingOption,
    NumOutOfRange,
    BadNumValue(String),
    BadUserDefinedPattern(String),
    BadGender(String),
    BadInflectCall(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadChunkingOption => write!(f, "group must be 0, 1, 2 or 3"),
            Error::NumOutOfRange => write!(f, "number out of range"),
            Error::BadNumValue(value) => write!(f, "bad numeric value: {value:?}"),
            Error::BadUserDefinedPattern(pattern) => {
                write!(f, "bad user-defined pattern: {pattern:?}")
            }
            Error::BadGender(gender) => write!(f, "unknown gender: {gender:?}"),
            Error::BadInflectCall(call) => write!(f, "cannot evaluate inflect call: {call:?}"),
        }
    }
}

impl std::error::Error for Error {}
