use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::fs::File;
use std::io::{self, BufReader};

pub fn compress(source: &str, target: &str) -> io::Result<()> {
    let mut input = BufReader::new(File::open(source)?);
    let output = File::create(target)?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    io::copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

pub fn decompress(source: &str, target: &str) -> io::Result<()> {
    let input_file = File::open(source)?;
    let mut decoder = GzDecoder::new(input_file);
    let mut output = File::create(target)?;
    io::copy(&mut decoder, &mut output)?;
    Ok(())
}
