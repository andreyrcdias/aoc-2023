use regex::Regex;
use std::fs::read_to_string;

fn part1(lines: &Vec<&str>) -> u32 {
    // lines.iter().map(|line| {
    //     let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
    //     let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    //     first * 10 + last
    // }).sum();

    // approach using regex
    let digit_pattern = Regex::new(r"\d").unwrap();
    lines
        .iter()
        .map(|line| {
            let first = digit_pattern
                .find(line)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            let last = digit_pattern
                .find(&line.chars().rev().collect::<String>())
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            first * 10 + last
        })
        .sum()
}

fn part2(_lines: &Vec<&str>) -> u32 {
    todo!();
}

fn main() {
    let file_path = "input.txt";
    let content = read_to_string(file_path).expect(&format!("Unable to read {}", file_path));
    let lines: Vec<&str> = content.lines().collect();
    println!("part 1 = {:?}", part1(&lines));
    // println!("part 2 = {:?}", part2(&lines));
}
