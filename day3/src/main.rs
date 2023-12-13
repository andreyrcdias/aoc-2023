use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let symbol_regex = Regex::new(r"[^.\d]").unwrap();
    let mut adjacent_symbols: HashSet<(usize, usize)> = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for m in symbol_regex.find_iter(line) {
            let j = m.start();
            for row in i.saturating_sub(1)..=i + 1 {
                for c in j.saturating_sub(1)..=j + 1 {
                    adjacent_symbols.insert((row, c));
                }
            }
        }
    }

    let mut sum = 0;
    let number_regex = Regex::new(r"\d+").unwrap();
    for (i, line) in lines.iter().enumerate() {
        for m in number_regex.find_iter(line) {
            let (start, end) = (m.start(), m.end());
            if (start..end).any(|j| adjacent_symbols.contains(&(i, j))) {
                sum += m.as_str().parse::<u32>().unwrap();
            }
        }
    }
    sum
}

fn main() {
    const FILE_PATH: &str = "input.txt";
    let input =
        read_to_string(FILE_PATH).unwrap_or_else(|_| panic!("Unable to read {}", FILE_PATH));
    println!("part 1 = {:?}", part1(&input));
}
