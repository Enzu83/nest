use std::fs::File;
use std::io::Read;

fn main() {
    // open the ROM and store the content in a byte vector
    let mut f = File::open("roms/mario.nes").expect("No file found at given path.");
    let mut data = vec![];
    f.read_to_end(&mut data).expect("Error while reading file.");

    // get the first three elements of the ROM
    let header = &data[..3];
    let header: Vec<char> = header.iter().map(|&b| b as char).collect();

    // the header should be "NES"
    println!("{}", header.iter().collect::<String>());
}