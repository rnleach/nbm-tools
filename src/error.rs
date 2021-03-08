/// A general error type.
#[derive(Debug)]
pub enum Error {
    /// A general error with a message describing it.
    General(String),
    /// Any other error is passed up this way.
    Internal(Box<dyn std::error::Error>),

    /// The NBMData doesn't have a matching column
    NoSuchColumn,
}

impl Error {
    /// Create a new error.
    pub fn general_error(msg: String) -> Self {
        Error::General(msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::General(msg) => write!(f, "{}", msg),
            Self::Internal(err) => write!(f, "{}", err),
            Self::NoSuchColumn => write!(f, "no such column"),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::Internal(err) => Some(err.as_ref()),
            _ => None,
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Internal(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}
