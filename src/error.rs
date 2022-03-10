use std::{error::Error as StdError, fmt};

/// An error type to wrap all errors that force us to terminate execution.
#[derive(Debug)]
pub struct Error {
    reason: String,
    cause: Option<Box<dyn StdError + 'static>>,
}

impl Error {
    pub fn with_cause<R, C>(reason: R, cause: C) -> Self
    where
        R: Into<String>,
        C: StdError + 'static,
    {
        Self {
            reason: reason.into(),
            cause: Some(Box::new(cause)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {:?}, Cause: {:?}", self.reason, self.cause)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.cause {
            Some(ref cause) => Some(&**cause),
            None => None,
        }
    }
}

impl From<csv::Error> for Error {
    fn from(other: csv::Error) -> Self {
        Self::with_cause("CSV Error", other)
    }
}
