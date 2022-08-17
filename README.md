
## XML RPC multi-threaded Bruteforce Tool

This is a simple implementation of an XML RPC API bruteforce tool that runs in multi-thread mode.

  

XML RPC is Application Program Interface that allows users to execute remote procedure calls by using a simple `username` and `password` authentication. Usually these APIs are hidden due to malicious operations executable by an attacker but in many CMS like Wordpress they are active by default.

  

## Installation

This tool has been written in Rust programming language so before running it you must ensure to have Rust toolkit installed

on your pc.

  

Please follow the instructions here: [Install Rust](https://www.rust-lang.org/tools/install).

  

## Usage

To run the tool:

```

git clone git@github.com:gnapoli23/xmlrpc-bf.git

cd xmlrpc-bf/

```

and then type

`cargo run --release -- --help`

  

The output will be:

```

A multi-threaded brute force tool to execute XMLRPC API attacks

  

USAGE:

xmlrpc-bf [OPTIONS] --username <USERNAME> --dictionary <FILE.txt> --xml-url <XML_URL>

  

OPTIONS:

-u, --username <USERNAME> Username to use as target for the attack [required]

-d, --dictionary <FILE.txt> Dictionary containing the passwords for the attack [required]

-x, --xml-url <XML_URL> XML RPC API url for the attack [required]

-t, --threads-num <THREADS_NUM> Number of threads to run [optional, default=5]

-h, --help Print help information

-V, --version Print version information

```