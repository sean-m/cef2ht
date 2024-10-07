use cef2hashmap::CefToHashMap;
use clap::Parser;
use flate2::read::GzDecoder;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

/// It might be faster than Python
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Expect stdin to be gzip compressed
    #[arg(long, short, default_value_t = false)]
    gunzip: bool,

    /// Preserve original event in rawEvent key
    #[arg(long, short, default_value_t = false)]
    preserve: bool,

    /// Comma delimited list of fields to select on
    #[arg(short, long, default_value_t = String::new())]
    fields: String,

    /// Redirect errors to stdout. For systems that regard any stderr output as a failure.
    #[arg(long, default_value_t = false)]
    stdout: bool,

    /// Prefix error lines for parsing downstream
    #[arg(short, long, default_value_t = String::new())]
    errprefix: String,
}

fn parse_n_dump(line: std::io::Result<String>, preserve: bool, err_to_std: bool, error_prefix: String, fields: &Vec<String>) {
        
    match line {
        Ok(l) => match l.to_hashmap(preserve) {
            Ok(o) => {
                let mut filtered = HashMap::new();
                if fields.len() < 1 { filtered = o }
                else {
                    o.iter()
                        .for_each(|(k, v)| 
                        if fields.contains(&k) { 
                            filtered.insert(k.clone(), v.to_string()); 
                        });
                }
                println!("{:?}", filtered)
            }
            Err(_e) => {
                if err_to_std { println!("{} {}", error_prefix, l); } 
                else { eprintln!("{} {}", error_prefix, l) }
            }
        },
        Err(e) => {
            eprintln!("Error readling line from stream: {:?}", e)
        }
    }
}

fn main() {
    let args = Args::parse();
    let use_gzip = args.gunzip;
    let preserve = args.preserve;
    let mut fields: Vec<String> = vec![];

    for f in args.fields.split(",") {
        if !f.is_empty() { fields.push(f.to_string().trim().to_owned()); }
    }

    let mut prefix = args.errprefix;
    if prefix.is_empty() {
        prefix = "Error parsing CEF line: ".to_owned();
    }

    if use_gzip {
        let gunzipped = GzDecoder::new(std::io::stdin());
        let reader = BufReader::new(gunzipped);
        reader
            .lines()
            .for_each(|line| parse_n_dump(line, preserve, args.stdout, prefix.clone(), &fields));
        return;
    }

    let reader = BufReader::new(std::io::stdin());
    reader
        .lines()
        .for_each(|line| parse_n_dump(line, preserve, args.stdout, prefix.clone(), &fields));
}
