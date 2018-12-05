use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let answer = get_answer(filename);

    println!("{}", answer);
}

fn get_answer(filename: &str) -> usize {
    let file_content_string = read_file(filename);
    let file_content = file_content_string.as_str();

    let pre_cleaned = reduce(&file_content);
    let tests = [
        reduce(remove_unit(pre_cleaned.as_str(), 'A').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'B').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'C').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'D').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'E').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'F').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'G').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'H').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'I').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'J').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'K').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'L').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'M').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'N').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'O').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'P').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'Q').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'R').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'S').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'T').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'U').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'V').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'W').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'X').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'Y').as_str()).len() - 1,
        reduce(remove_unit(pre_cleaned.as_str(), 'Z').as_str()).len() - 1
    ];

    *tests.iter().min().unwrap()
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

fn remove_unit(polymer: &str, uppercase: char) -> String {
    let lowercase = uppercase.to_ascii_lowercase();
    polymer.replace(lowercase, "").replace(uppercase, "")
}