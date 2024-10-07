use std::io::{prelude::*, BufReader};
use flate2::read::GzDecoder;
use clap::Parser;
use cef2hashmap::CefToHashMap;

/// It might be faster than Python
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Expect stdin to be gzip compressed
    #[arg(long, short, default_value_t = false)]
    gunzip: bool,

    /// Preserve event in rawEvent key
    #[arg(long, short, default_value_t = false)]
    preserve: bool,
}

fn parse_n_dump (line : std::io::Result<String>, preserve : bool) {
    match line {
        Ok(l) => {    
            match l.to_hashmap(preserve) {
                Ok(o) => { println!("{:?}", o) }
                Err(e) => { eprintln!("Error parsing cef to hashmap: {:?}", e)}
            }
        }
        Err(e) => { eprintln!("Error readling line from stream: {:?}", e)}
    }
}

fn main() {
    let args = Args::parse();
    let use_gzip = args.gunzip;
    let preserve = args.preserve;
    
    if use_gzip {
        let gunzipped = GzDecoder::new(std::io::stdin());
        let reader = BufReader::new(gunzipped);
        reader.lines().for_each(|line| parse_n_dump(line, preserve));
        return;
    }

    let reader = BufReader::new(std::io::stdin());
    reader.lines().for_each(|line| parse_n_dump(line, preserve));
}