use std::collections::HashSet;
use std::fs;

struct Card {
    winning_numbers: HashSet<i64>,
    chosen_numbers: HashSet<i64>,
}

impl Card {
    fn count(&self) -> usize {
        self.winning_numbers
            .intersection(&self.chosen_numbers)
            .count()
    }

    fn score(&self) -> i64 {
        if self.count() > 0 {
            1 << (self.count() - 1)
        } else {
            0
        }
    }
}

fn extract_cards(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let (_, nums) = line.split_once(": ").unwrap();
        let (win, chose) = nums.split_once(" | ").unwrap();
        let winning_numbers = win
            .split_whitespace()
            .map(|snum| snum.parse::<i64>().unwrap())
            .collect();
        let chosen_numbers = chose
            .split_whitespace()
            .map(|snum| snum.parse::<i64>().unwrap())
            .collect();
        cards.push(Card {
            winning_numbers,
            chosen_numbers,
        });
    }
    cards
}

fn part1(input: &str) -> i64 {
    let cards = extract_cards(input);
    cards.iter().map(Card::score).sum::<i64>()
}

fn part2(input: &str) -> usize {
    let cards = extract_cards(input);

    let mut multiplier = vec![1usize; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let count = card.count();
        for i in index + 1..index + 1 + count {
            multiplier[i] += multiplier[index];
        }
    }

    multiplier.iter().sum::<usize>()
}

fn main() {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH).expect("Unable to open file");
    println!("part 1 = {:?}", part1(&input));
    println!("part 2 = {:?}", part2(&input));
}
