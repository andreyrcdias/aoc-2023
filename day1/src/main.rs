use regex::Regex;
use std::fs;

use aho_corasick::AhoCorasick;

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .iter()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            first * 10 + last
        })
        .sum()
}

fn part1_regex(input: &str) -> i32 {
    // approach using regex
    let lines: Vec<&str> = input.lines().collect();

    let digit_pattern = Regex::new(r"\d").unwrap();
    lines
        .iter()
        .map(|line| {
            let first = digit_pattern
                .find(line)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let last = digit_pattern
                .find(&line.chars().rev().collect::<String>())
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            first * 10 + last
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let total: u32 = input
        .lines()
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2twooo")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| {
            let first = vec.first().unwrap();
            let last = vec.last().unwrap();
            10 * first + last
        })
        .sum();

    total.try_into().unwrap()
}

fn part2_aho_corasick(input: &str) -> i32 {
    let nums = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    let ac = AhoCorasick::new(nums).unwrap();

    let total = input
        .lines()
        .map(|line| {
            let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
            let first = matches.first().unwrap().pattern().as_i32() / 2 + 1;
            let last = matches.last().unwrap().pattern().as_i32() / 2 + 1;
            10 * first + last
        })
        .sum();

    total
}

fn main() {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH).expect("Unable to open file");
    println!("part 1 = {:?}", part1(&input));
    println!("part 1 regex = {:?}", part1_regex(&input));

    println!("part 2 = {:?}", part2(&input));
    println!("part 2 AHO Corasick = {:?}", part2_aho_corasick(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const FILE_PATH: &str = "sample.txt";
        let input = fs::read_to_string(FILE_PATH).expect("Unable to open file");
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn test_part1_regex() {
        const FILE_PATH: &str = "sample.txt";
        let input = fs::read_to_string(FILE_PATH).expect("Unable to open file");
        assert_eq!(part1_regex(&input), 142);
    }

    #[test]
    fn test_part2() {
        const FILE_PATH: &str = "sample2.txt";
        let input = fs::read_to_string(FILE_PATH).expect("Unable to open file");
        assert_eq!(part2(&input), 281);
    }

    #[test]
    fn test_part2_aho_corasick() {
        const FILE_PATH: &str = "sample2.txt";
        let input = fs::read_to_string(FILE_PATH).expect("Unable to open file");
        assert_eq!(part2_aho_corasick(&input), 281);
    }
}
