use std::fmt::Display;


pub enum Error<'a> {
    UnableToParseArg(&'a str), // Generic error to handle every cli args error
    PathDoesNotExist,
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnableToParseArg(arg) => write!(f, "Unable to parse command line argument: {:?}", arg),
            Error::PathDoesNotExist => write!(f, "File path does not exist"),
        }
    }
}

impl<'a> From<&'a str> for Error<'a> {
    fn from(arg: &'a str) -> Self {
        Error::UnableToParseArg(arg)
    }
}

impl<'a> From<Error<'a>> for String {
    fn from(e: Error) -> Self {
        e.to_string()
    }
}