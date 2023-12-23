//use regex::Regex;
use std::fs;

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

    // approach using regex
    // let digit_pattern = Regex::new(r"\d").unwrap();
    // lines
    //     .iter()
    //     .map(|line| {
    //         let first = digit_pattern
    //             .find(line)
    //             .unwrap()
    //             .as_str()
    //             .parse::<u32>()
    //             .unwrap();
    //         let last = digit_pattern
    //             .find(&line.chars().rev().collect::<String>())
    //             .unwrap()
    //             .as_str()
    //             .parse::<u32>()
    //             .unwrap();
    //         first * 10 + last
    //     })
    //     .sum()
}

fn part2(input: &str) -> u32 {
    input
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
        .sum()
}

fn main() {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH).expect("Unable to open file");
    println!("part 1 = {:?}", part1(&input));
    println!("part 2 = {:?}", part2(&input));
}
