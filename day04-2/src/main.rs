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
    let line_re = Regex::new(r"(?m)^\[(\d{4}-\d{2}-\d{2}) (\d{2}):(\d{2})\] (.+)$").unwrap();
    let shift_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let mut current_guard: usize = 0;
    let mut guards_asleep = HashMap::new();
    let mut asleep_at: usize = 0;

    for line in line_re.captures_iter(file_content) {
        let current_min: usize = (&line[3]).parse().unwrap();
        match &line[4] {
            "wakes up" => {
                for m in asleep_at..current_min {
                    let guard_asleep: &mut HashMap<usize, usize> = guards_asleep
                        .entry(current_guard)
                        .or_insert(HashMap::new());
                    let minute_count = guard_asleep.entry(m).or_insert(0);
                    *minute_count += 1;
                }
            },
            "falls asleep" => {
                asleep_at = current_min;
            },
            _ => {
                let guard = shift_re.captures(&line[4]).unwrap();
                current_guard = guard[1].parse().unwrap();
            }
        }
    }

    let mut max_asleep_guard: usize = 0;
    let mut max_asleep_min: usize = 0;
    let mut max_asleep_time: usize = 0;

    for (guard_no, guard_asleep) in guards_asleep {
        for (min, time_asleep) in guard_asleep {
            if time_asleep > max_asleep_time {
                max_asleep_guard = guard_no;
                max_asleep_min = min;
                max_asleep_time = time_asleep;
            }
        }
    }

    max_asleep_guard * max_asleep_min

}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    return contents
}