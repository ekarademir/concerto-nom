use nom::error::{ContextError, ErrorKind, FromExternalError, ParseError};

/// Errors occuring during parse operations, Concerto parse error type
#[derive(Debug, PartialEq)]
pub struct CError<I> {
    /// Concerto parse error code
    pub code: CErrorKind,
    /// Position of the error
    pub input: I,
}

impl<I: std::fmt::Display> std::fmt::Display for CError<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error {:?} at: {}", self.code, self.input)
    }
}

impl<I: std::fmt::Debug + std::fmt::Display> std::error::Error for CError<I> {}

impl<I: std::fmt::Debug + std::fmt::Display> ParseError<I> for CError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        match kind {
            ErrorKind::Space => {
                let mut found = input.to_string();
                found.truncate(5);
                Self {
                    code: CErrorKind::ExpectedFound(String::from("Space"), found),
                    input,
                }
            }
            _ => Self {
                code: CErrorKind::NomError(kind),
                input,
            },
        }
    }

    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> ContextError<I> for CError<I> {}

impl<I, E> FromExternalError<I, E> for CError<I> {
    fn from_external_error(input: I, kind: ErrorKind, _e: E) -> Self {
        CError {
            code: CErrorKind::NomError(kind),
            input,
        }
    }
}

/// Kinds of errors while parsing Concerto files
#[derive(Debug, PartialEq)]
pub enum CErrorKind {
    /// Surfacing nom errors
    NomError(ErrorKind),
    /// Expected token
    ExpectedFound(String, String),
    /// String property meta
    StringPropertyWrongMeta,
    /// With context
    Context(&'static str),
}
