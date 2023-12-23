use std::collections::HashSet;
use std::fs;

struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            // right
            (row, col + 1),
            (row + 1, col + 1),
            // above + below
            (row - 1, col),
            (row + 1, col),
            (row - 1, col + 1),
            // left
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1),
        ]);
        Self {
            value: (ch as u8 - b'0') as i64,
            points,
        }
    }

    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value = self.value * 10 + (ch as u8 - b'0') as i64;
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }

    fn extract_value(&self) -> i64 {
        self.value
    }

    fn next_to_symbol(&self, symbols: &HashSet<(i64, i64)>) -> bool {
        self.points.intersection(symbols).next().is_some()
    }
}

struct Data {
    numbers: Vec<PartNumber>,
    symbols: HashSet<(i64, i64)>,
    gears: HashSet<(i64, i64)>,
}

impl Data {
    pub fn new(
        numbers: Vec<PartNumber>,
        symbols: HashSet<(i64, i64)>,
        gears: HashSet<(i64, i64)>,
    ) -> Self {
        Self {
            numbers,
            symbols,
            gears,
        }
    }
}

fn parse_input(input: &str) -> Data {
    let lines: Vec<&str> = input.lines().filter(|&s| !s.trim().is_empty()).collect();

    let mut numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: HashSet<(i64, i64)> = HashSet::new();
    let mut gears: HashSet<(i64, i64)> = HashSet::new();

    let mut cur_number: Option<PartNumber> = None;

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = cur_number {
                    num.add_digit(row as i64, col as i64, ch);
                } else {
                    cur_number = Some(PartNumber::new(row as i64, col as i64, ch));
                }
            } else {
                if let Some(num) = cur_number.take() {
                    numbers.push(num);
                }
                if ch != '.' {
                    symbols.insert((row as i64, col as i64));
                    if ch == '*' {
                        gears.insert((row as i64, col as i64));
                    }
                }
            }
        }
    }

    Data::new(numbers, symbols, gears)
}

fn part1(input: &str) -> i64 {
    let data = parse_input(input);
    let total = data
        .numbers
        .iter()
        .filter(|num| num.next_to_symbol(&data.symbols))
        .map(PartNumber::extract_value)
        .sum::<i64>();
    total
}

fn part2(input: &str) -> i64 {
    let data = parse_input(input);
    let mut total = 0;
    'next_gear: for gear in &data.gears {
        let mut matches = Vec::new();
        for num in &data.numbers {
            if num.points.contains(gear) {
                if matches.len() == 2 {
                    continue 'next_gear;
                }
                matches.push(num.value);
            }
        }
        if matches.len() == 2 {
            total += matches[0] * matches[1];
        }
    }
    total
}

fn main() {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH).expect("Unable to open file");
    println!("part 1 = {:?}", part1(&input));
    println!("part 2 = {:?}", part2(&input));
}
