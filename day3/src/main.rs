use std::fs::read_to_string;
use std::collections::HashSet;
use regex::Regex;

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let symbol_regex = Regex::new(r"[^.\d]").unwrap();
    let mut symbol_adjacent: HashSet<(usize, usize)> = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for m in symbol_regex.find_iter(line) {
            let j = m.start();
            for row in i.saturating_sub(1)..=i + 1 {
                for c in j.saturating_sub(1)..=j + 1 {
                    symbol_adjacent.insert((row, c));
                }
            }
        }
    }

    let number_regex = Regex::new(r"\d+").unwrap();
    let mut part_num_sum = 0;
    for (i, line) in lines.iter().enumerate() {
        for m in number_regex.find_iter(line) {
            let (start, end) = (m.start(), m.end());
            if (start..end).any(|j| symbol_adjacent.contains(&(i, j))) {
                part_num_sum += m.as_str().parse::<u32>().unwrap();
            }
        }
    }
    part_num_sum
}

fn main() {
    const FILE_PATH: &str = "input.txt";
    let input =
        read_to_string(FILE_PATH).unwrap_or_else(|_| panic!("Unable to read {}", FILE_PATH));
    println!("part 1 = {:?}", part1(&input));
}
