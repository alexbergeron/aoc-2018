use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file_content = read_file(filename);
    let lines = file_content.split("\n").filter(|str| !str.is_empty());

    let checksum = get_checksum(lines);

    println!("{}", checksum);
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    return contents
}

fn get_checksum<'a, I>(lines: I) -> usize 
    where I: Iterator<Item=&'a str> {
    let mut twice_count: usize = 0;
    let mut thrice_count: usize = 0;
    for line in lines {
        let mut has_twice = false;
        let mut has_thrice = false;
        for value in freq(line.as_bytes()).values() {
            if *value == 2 && !has_twice {
                has_twice = true;
                twice_count += 1;
            } else if *value == 3 && !has_thrice {
                has_thrice = true;
                thrice_count += 1;
            }
            if has_twice && has_thrice {
                break;
            }
        }
    }
    twice_count * thrice_count
}

fn freq(line: &[u8]) -> HashMap<&u8, u8> {
    let mut frequencies = HashMap::new();
    for b in line {
        let count = frequencies.entry(b).or_insert(0);
        *count += 1;
    }
    frequencies
}