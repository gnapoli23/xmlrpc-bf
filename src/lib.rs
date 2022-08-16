use std::ops::RangeInclusive;

use clap::Parser;
mod errors;
use errors::Error;
use tracing::error;

const THREADS_NUM_RANGE: RangeInclusive<i64> = 1..=10;
const CALLS_NUM_RANGE: RangeInclusive<i64> = 1..=1000;

// Arguments definition
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct CmdArgs {
    #[clap(short = 'u', long)]
    pub username: String,
    #[clap(short = 'd', long)]
    pub dictionary: String,
    #[clap(short = 'x', long)]
    pub xml_url: String,
    #[clap(short = 't', long, value_parser = clap::value_parser!(u8).range(THREADS_NUM_RANGE))]
    pub threads_num: Option<u8>,
    #[clap(short = 'c', long, value_parser = clap::value_parser!(u8).range(CALLS_NUM_RANGE))]
    pub calls_num: Option<u8>,
    #[clap(long = "debug", action)]
    pub debug_mode: bool,
}

pub fn main() -> Result<(), Error> {

    // Parsing cmd line args
    let args = CmdArgs::try_parse().map_err(|e| {
        error!("Unable to parse command line arguments");
        Error::UnableToParseArgs(e)
    })?;
    
    todo!()
}
