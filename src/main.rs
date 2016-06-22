extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Vat calculator.

Usage:
  vat [-r] <value> [--rate=<rate>]
  vat (-h | --help)
  vat --version

Options:
  -r                  Compute from value without tax to value with tax.
  --rate=<rate>       Change vat rate [default: 20]
  --version           Get version
  -h --help           Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version: bool,
    flag_rate: f32,
    flag_r: bool,
    arg_value: Option<f32>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    // Version?
    if args.flag_version {
        println!("vatax v{}", VERSION);
        std::process::exit(0);
    }

    // Get value
    let value = match args.arg_value {
        Some(x) => x,
        None    => 0.0,
    };

    // Compute, please.
    let rate = 1.0 + args.flag_rate/ 100.0;
    let ( wt, wot ) = if args.flag_r {
        (value, value / rate)
    } else {
        (value * rate, value)
    };
    println!("Without tax {}\nWith tax    {}", format!("{:.*}", 2, wot), format!("{:.*}", 2, wt));      
           
}