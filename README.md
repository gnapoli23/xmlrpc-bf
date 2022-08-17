
## XML RPC multi-threaded Bruteforce Tool

This is a simple implementation of an XML RPC API bruteforce tool that runs in multi-thread mode, and it's been inspired from this article: [Exploiting the XML RPC PHP on all Wordpress Versions](https://nitesculucian.github.io/2019/07/01/exploiting-the-xmlrpc-php-on-all-wordpress-versions/)


XML RPC is Application Program Interface that allows users to execute remote procedure calls by using a simple `username` and `password` authentication. Usually these APIs are hidden due to malicious operations executable by an attacker but in many CMS like Wordpress they are active by default.

  

## Installation

This tool has been written in Rust programming language so before running it you must ensure to have Rust toolkit installed on your pc.
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

## Improvements

There are differente improvements that could be done:
- use of `system.multicall` instead of sending `n` single request. The initial implementation of the tool was based on this particulare RPC API but during development i found that it was difficult to share information info about a possible match found. The crate `xmlrpc` on which this tool is based unfortunately does not provide a mean to map each request sent through `system.multicall` to its relative response.
- add an optional argument to run the requests through TOR Network
- add cli auto-completion using [clap_complete](https://crates.io/crates/clap_complete)
- add file reading also for target username

Please feel free to suggest any other change to the tool, any advice is well accepted!

## License

Copyright © 2022 Giovanni Napoli [gnapoli.dev@gmail.com](mailto:gnapoli.dev@gmail.com)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

## Donation

If you liked the project and want to support, i really thank you!

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/gnapoli23)
