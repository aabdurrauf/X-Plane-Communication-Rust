use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};

fn main() {
    // Create a buffer to store the packed data
    let mut buffer = Vec::new();

    // Pack the data into the buffer
    buffer.write_all(b"GETP").unwrap(); // Writes the string "GETP"
    buffer.write_i8(0).unwrap();        // Writes a single byte with value 0

    // Print the packed data (for demonstration)
    println!("{:?}", buffer);
}
