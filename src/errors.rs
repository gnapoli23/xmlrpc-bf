use std::fmt::Display;


pub enum Error {
    UnableToParseArgs(clap::Error), // Generic error to handle every cli args error
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::UnableToParseArgs(e) => write!(f, "Unable to parse command line arguments: {:?}", e),
        }
    }
}

impl From<clap::Error> for Error {
    fn from(e: clap::Error) -> Self {
        Error::UnableToParseArgs(e)
    }
}