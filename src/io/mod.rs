use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::Write;
use arrayvec::ArrayString;

pub type Comment = ArrayString<[u8; 32]>;
pub fn blank_comment() -> Comment { Comment::new() }

#[derive(Copy, Clone)]
pub struct FileLine {
    pub value: u8,
    pub comment: Comment,
}

fn extract_values_from_line(line: &String) -> FileLine {
    let mut lvec = line.split(" ; ").collect::<Vec<_>>();

    if lvec.len() == 1 { // if this line don't have a comment, set a blank value
        lvec.push("");
    }

    FileLine {
        value: u8::from_str_radix(&lvec[0], 16).unwrap(),
        comment: Comment::from(lvec[1]).unwrap(),
    }
}

pub fn load_file(filepath: &str) -> [FileLine; 255] {
    let mut output: [FileLine; 255] = [
        FileLine { value: 0x00, comment: blank_comment() }; 255
    ];

    let f = File::open(filepath).expect("File not found!");
    let file = BufReader::new(&f);
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        output[num] = extract_values_from_line(&l);
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

pub fn save_memory(filepath: &str, memory: [u8; 255]) {
    let mut f = File::create(filepath).expect("Error!");

    let mut content_placeholder: Vec<Vec<u8>> = vec![vec![]; 255];

    for (num, line) in memory.iter().enumerate() {
        content_placeholder[num] = format!("{:02X?}\n", line).as_bytes().to_vec();
    }

    let content_flatten: Vec<u8> = content_placeholder
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect();

    f.write_all(content_flatten.as_slice()).expect("Error while saving!");
}
