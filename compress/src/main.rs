extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    // Verify correct number of command-line arguments
    if args().len() != 3 {
        eprintln!("Usage: 'source' 'target'");
        return;
    }
   
    // Open input file and wrap in buffered reader
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // Create output file for compressed data
    let output = File::create(&args().nth(2).unwrap()).unwrap();
    // Initialize gzip encoder with default compression level
    let mut encoder = GzEncoder::new(output, Compression::default());
    
    // Start timer
    let start = Instant::now();
    // Copy and compress input to output
    copy(&mut input, &mut encoder).unwrap(); 
    // Finish encoding and get the file handle
    let output = encoder.finish().unwrap();
    // End timer
    let end = Instant::now();
    
    // Print compression time
    eprintln!("Compression time: {:?}", end - start);
    // Print compressed file size
    println!("source len:{:?}", output.metadata().unwrap().len()); 
}