use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let common = get_common(filename);

    println!("{}", common);
}

fn get_common(filename: &str) -> String {
    let file_content = read_file(filename);
    let lines: Vec<&str> = file_content.split("\n").filter(|str| !str.is_empty()).collect();

    for (index, a_line) in lines.iter().enumerate() {
        for b_line in &lines[index..] {
            let current = one_apart_at(a_line, b_line);
            if !current.is_empty() {
                return current;
            }
        }
    }
    panic!();
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    return contents
}

fn one_apart_at<'a>(a: &'a str, b: &'a str) -> String {
    let mut found = false;
    let mut answer = String::new(); 
    
    for (a_char, b_char) in a.chars().zip(b.chars()) {
        if a_char != b_char && !found {
            found = true
        } else if a_char != b_char {
            return String::new();
        } else {
            answer.push(a_char);
        }
    }

    if found {
        answer
    } else {
        String::new()
    }
}