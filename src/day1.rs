use super::utils::read_file_lines;

use phf::{phf_map};

static NUM_WORDS: phf::Map<&'static str, &'static str> = phf_map! {
    "nine" => "n9ne",
    "eight" => "ei8ht",
    "seven" => "se7en",
    "six" => "s6x",
    "five" => "f5ve",
    "four" => "f4ur",
    "three" => "th3ee",
    "two" => "t2o",
    "one" => "o1n",
};

fn handle_line(line: &str) -> u64 {
    let mut first = 255;
    let mut last = 255;
    for c in line.chars() {
        let v = c as u32 - '0' as u32;
        if v > 0 && v < 10 {
            if first == 255 {
                first = v
            }
            last = v
        }
    }

    return format!("{}{}", first, last).parse::<u64>().unwrap();
}

pub fn solve_part_1() {
    let lines = read_file_lines("inputs/day1").expect("pyyzdec");
    let mut res = 0;
    for line in lines {
        res += handle_line(&line);
    }
    println!("Part 1 answer is: {}", res)
}

pub fn solve_part_2() {
    let lines = read_file_lines("inputs/day1").expect("pyyzdec");
    let mut res = 0;
    for mut line in lines {
        for (key, value) in &NUM_WORDS {
            line = line.replace(key, value);
        } 
        res += handle_line(&line);
    }
    println!("Part 2 answer is: {}", res)
}