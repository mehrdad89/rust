use flate2::Compression;
use flate2::write::DeflateEncoder;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Get the name of the input file from the command line arguments
    let args: Vec<String> = std::env::args().collect();
    let input_file_name = &args[1];

    // Open the input file for reading
    let mut input_file = File::open(input_file_name)?;

    // Create a new file for the compressed output
    let output_file_name = format!("{}.deflate", input_file_name);
    let mut output_file = File::create(output_file_name)?;

    // Create a DEFLATE encoder and write the compressed data to the output file
    let mut encoder = DeflateEncoder::new(&mut output_file, Compression::default());
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = input_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        encoder.write_all(&buffer[..bytes_read])?;
    }
    encoder.finish()?;
    
    Ok(())
}
