use clap::Parser;
mod errors;
use errors::Error;
use tracing::error;

// Arguments definition
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct CmdArgs {
    pub username: String,
    pub dictionary: String,
    pub xml_url: String,
    pub threads_num: Option<u8>,
    pub calls_num: Option<u8>,
    pub debug_mode: Option<bool>,
}

fn parse_args() -> Result<CmdArgs, errors::Error> {
    Ok(CmdArgs::try_parse().map_err(|e| {
        error!("Unable to parse command line arguments");
        Error::UnableToParseArgs(e)
    })?)
}

pub fn main() -> Result<(), Error> {

    // Parsing cmd line args
    let args = parse_args()?;
    todo!()
}
