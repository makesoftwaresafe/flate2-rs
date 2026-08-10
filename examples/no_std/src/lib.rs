#![no_std]
#![cfg_attr(flate2_unstable_nightly_alloc_io, feature(alloc_io))]

extern crate alloc;

cfg_select! {
    flate2_unstable_nightly_alloc_io => {
        use alloc::io;
        use log::info as println;
    }
    _ => {
        extern crate std;
        use std::io;
        use std::println;
    }
}

use alloc::string::String;
use alloc::vec::Vec;
use io::prelude::*;

use flate2::write::GzEncoder;
use flate2::{read, Compression};

// Compress a sample string and print it after transformation.
pub fn main() {
    let input = "Hello World";

    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(input.as_bytes()).unwrap();
    let bytes = e.finish().unwrap();
    println!("Compressed '{input}' into: {bytes:?}");

    let output = decode_reader(bytes).unwrap();
    println!("Decompressed into: {output}");

    assert_eq!(input, &output);
}

// Uncompresses a Gz-encoded vector of bytes and returns a string or error
// Here &[u8] implements the Read trait
fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut gz = read::GzDecoder::new(&bytes[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    Ok(s)
}
