use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut frequency: isize = 0;
    let mut repeated = HashSet::new();
    'outer: loop {
        for nb in content_to_intarr(contents.as_str()) {
            frequency += nb;
            if repeated.contains(&frequency) {
                break 'outer;
            }
            repeated.insert(frequency);
        }
    }

    println!("{}", frequency);
}

fn content_to_intarr(content: &str) -> Vec<isize> {
    content
        .split("\n")
        .filter(|str| !str.is_empty())
        .map(|str| str.parse::<isize>().expect(format!("Invalid input {}", str).as_str()))
        .collect()
}