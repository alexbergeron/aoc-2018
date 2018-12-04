extern crate regex;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let answer = get_answer(filename);

    println!("{}", answer);
}

fn get_answer(filename: &str) -> usize {
    let file_content_string = read_file(filename);
    let file_content = file_content_string.as_str();
    let re = Regex::new(r"#(\d{1,4}) @ (\d{1,3}),(\d{1,3}): (\d{1,2})x(\d{1,2})").unwrap();
    let mut pair_count = HashMap::new();

    for line in re.captures_iter(file_content) {
        let x0: usize = (&line[2]).parse().unwrap();
        let y0: usize = (&line[3]).parse().unwrap();
        let w: usize = (&line[4]).parse().unwrap();
        let h: usize = (&line[5]).parse().unwrap();

        for x in x0..x0+w {
            for y in y0..y0+h {
                let pair = (x, y);
                let count = pair_count.entry(pair).or_insert(0);
                *count += 1;
            }
        }
    }

    'outer: for line in re.captures_iter(file_content) {
        let c: usize = (&line[1]).parse().unwrap();
        let x0: usize = (&line[2]).parse().unwrap();
        let y0: usize = (&line[3]).parse().unwrap();
        let w: usize = (&line[4]).parse().unwrap();
        let h: usize = (&line[5]).parse().unwrap();

        for x in x0..x0+w {
            for y in y0..y0+h {
                let pair = (x, y);
                if pair_count.get(&pair) > Some(&1) {
                    continue 'outer;
                }
            }
        }
        return c;
    }
    panic!()
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    return contents
}