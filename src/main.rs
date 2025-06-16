mod formats;

use formats::gzip;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: {} <format> <command> <source> <target>", args[0]);
        return;
    }

    let format = &args[1];
    let command = &args[2];
    let source = &args[3];
    let target = &args[4];

    match format.as_str() {
        "gzip" => match command.as_str() {
            "compress" => gzip::compress(source, target).unwrap(),
            "decompress" => gzip::decompress(source, target).unwrap(),
            _ => eprintln!("Unknown command for Gzip: {}", command),
        },
        _ => eprintln!("Unknown format: {}", format),
    }
}
