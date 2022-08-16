use clap::Parser;
mod errors;
use errors::Error;
use tracing::error;

// Arguments definition
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct CmdArgs {
    #[clap(short, long)]
    pub username: String,
    #[clap(short, long)]
    pub dictionary: String,
    #[clap(short, long)]
    pub xml_url: String,
    #[clap(short, long)]
    pub threads_num: Option<u8>,
    #[clap(short, long)]
    pub calls_num: Option<u8>,
    #[clap(short, long)]
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
