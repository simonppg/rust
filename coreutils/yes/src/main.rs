extern crate getopts;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = getopts::Options::new();

    opts.optflag("h", "help", "show this information");
    opts.optflag("v", "version", "command version");

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") { print_help(); return; }
    if matches.opt_present("v") { print_version(); return; }

    let mut message: String = "".to_string();

    if args.len() > 1 {
        for arg in args[1..].iter() {
            message.insert_str(message.len(), arg);
            message.insert_str(message.len(), " ");
        }
    }

    if message.len() > 1 {
        loop {
            println!("{}", message);
        }
    }
    else {
        loop {
            println!("y")
        }
    }
}

fn print_help() {
    println!("Usage: yes [STRING]...");
    println!("or   : yes OPTIONS");
    println!("Repeatedly output a line with all specified STRING(s), or 'y'");
    println!("");
    println!("       -h, --help    : display this help and exit");
    println!("       -v, --version : output version information and exit");
}

fn print_version() {
    println!("v0.1.0");
}
