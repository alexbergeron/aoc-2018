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
    for line in contents.split("\n") {
        if !line.is_empty() {
            let current: isize = line.parse().unwrap();
            frequency += current;
        }
    }

    println!("{}", frequency);
}