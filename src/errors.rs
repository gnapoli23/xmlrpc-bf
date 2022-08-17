use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    UnableToParseArg(String),
    PathDoesNotExist(String),
    UnableToReadFile(std::io::Error),
    UnableToReadLine(std::io::Error),
    UnableToParseUrl(String),
    UnableToPerformRequest(xmlrpc::Error),
    GenericError,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::UnableToParseArg(arg) => {
                write!(f, "Unable to parse {:?} command line argument", arg)
            }
            Error::PathDoesNotExist(arg) => write!(f, "File path {:?} does not exist", arg),
            Error::UnableToReadFile(e) => write!(f, "Unable to read file: {:?}", e),
            Error::UnableToReadLine(e) => write!(f, "Unable to read line: {:?}", e),
            Error::UnableToParseUrl(arg) => write!(f, "Unable to parse {:?} as url", arg),
            Error::UnableToPerformRequest(e) => write!(f, "Unable to perform the request: {:?}", e),
            Error::GenericError => write!(f, "Generic error"),
        }
    }
}

impl From<String> for Error {
    fn from(arg: String) -> Self {
        Error::UnableToParseArg(arg)
    }
}

impl From<Error> for String {
    fn from(e: Error) -> Self {
        e.to_string()
    }
}
