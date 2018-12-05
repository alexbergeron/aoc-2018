use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let answer = get_answer(filename);

    println!("{}", answer.len() - 1);
}

fn get_answer(filename: &str) -> String {
    let file_content_string = read_file(filename);
    let file_content = file_content_string.as_str();

    reduce(&file_content)
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    contents
}

fn reduce(polymer: &str) -> String {
    match find_reaction(polymer) {
        None => String::from(polymer),
        Some(reaction_index) => {
            let before = &polymer[0..reaction_index];
            let after = &polymer[reaction_index+2..];
            let reducing = &[before, after].concat();
            reduce(reducing)
        }
    }
}

fn find_reaction(polymer: &str) -> Option<usize> {
    let end = polymer.len() - 1;
    let bytes = polymer.as_bytes();
    for (i, c) in bytes[..end].iter().enumerate() {
        let n_c = bytes.iter().nth(i + 1).unwrap();
        if (n_c > c && n_c - c == 32) || (c > n_c && c - n_c == 32) {
            return Some(i);
        }
    }
    None
}