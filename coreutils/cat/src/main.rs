extern crate getopts;
use std::env;
use std::io;
use std::fs::File;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let args:Vec<String> = env::args().collect();
    let opts = getopts::Options::new();
    let matches = opts.parse(&args[1..]).unwrap();

    let input:Result<Box<dyn BufRead>, io::Error> =
        matches.free.first().map_or(
            Ok(Box::new(io::stdin().lock())),
            |p| {
                Ok(Box::new(io::BufReader::new(File::open(p)?)))
            });
    let mut read = input.unwrap();
    loop {
        let mut buffer = String::new();
        match read.read_line(&mut buffer) {
            Ok(bytes) => match  bytes {
                0 => break,
                _ => print!("{}", &buffer),
            }
            Err(_) => break,
        }
    }
    Ok(())
}
