use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn load_file(filepath: &str) -> [u8; 255] {
    let mut output: [u8; 255] = [0x00; 255];

    let f = File::open(filepath).expect("File not found!");
    let file = BufReader::new(&f);
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        output[num] = u8::from_str_radix(&l, 16).unwrap();
    }

    output
}

pub fn load_inputs(string: &str) -> [u8; 255] {
    let mut output: [u8; 255] = [0x00; 255];

    if string == "" {
        return output
    }

    for (num, line) in string.split(" ").enumerate() {
        output[num] = u8::from_str_radix(&line, 16).unwrap();
    }

    output
}

