use std::ops::RangeInclusive;
use std::path::Path;
use url::Url;

use clap::{AppSettings, Parser};
mod errors;
use errors::Error;

const THREADS_NUM_RANGE: RangeInclusive<i64> = 1..=10;
const CALLS_NUM_RANGE: RangeInclusive<i64> = 1..=1000;

// Arguments definition
#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(allow_negative_numbers = false)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct CmdArgs {
    /// Username to use as target for the attack [required]
    #[clap(short = 'u', long)]
    pub username: String,
    /// Dictionary containing the passwords for the attack [required]
    #[clap(short = 'd', long, value_name = "FILE.txt", value_parser = parse_dictionary_path)]
    pub dictionary: String,
    /// XML RPC API url for the attack [required]
    #[clap(short = 'x', long, value_parser = parse_xml_url)]
    pub xml_url: String,
    /// Number of threads to run [optional, (min 1 - max 10) default=5]
    #[clap(short = 't', long, value_parser = clap::value_parser!(u8).range(THREADS_NUM_RANGE))]
    pub threads_num: Option<u8>,
    /// Number of system.multicall() calls to execute [optional, (min 1 - max 1000) default=100]
    #[clap(short = 'c', long, value_parser = clap::value_parser!(u8).range(CALLS_NUM_RANGE))]
    pub calls_num: Option<u8>,
    /// Verbose execution
    #[clap(short = 'v', long = "verbose", action)]
    pub verbose_mode: bool,
}

fn parse_dictionary_path(dictionary_arg: &str) -> Result<String, String> {
    Path::new(dictionary_arg)
        .try_exists()
        .map(|exst| {
            if exst {
                Ok(dictionary_arg.into())
            } else {
                Err(format!("Path '{}' does not exist", dictionary_arg))
            }
        })
        .map_err(|_| Error::UnableToParseArg(dictionary_arg))?
}

fn parse_xml_url(xml_url_arg: &str) -> Result<String, String> {
    Url::parse(xml_url_arg)
        .map(|url| Ok(url.into()))
        .map_err(|_| Error::UnableToParseUrl(xml_url_arg))?
}

pub fn main() {
    let args = CmdArgs::parse();

    println!("{:?}", args);
}
