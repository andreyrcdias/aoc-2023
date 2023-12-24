use std::fs;

struct RaceInfo {
    time: i64,
    record: i64,
}

impl RaceInfo {
    fn wins(&self) -> i64 {
        let a = -1f64;
        let b = self.time as f64;
        let c = (-self.record) as f64;

        let mut first = ((-b + (b * b - 4. * a * c).sqrt()) / (2. * a)).ceil();
        let mut second = ((-b - (b * b - 4. * a * c).sqrt()) / (2. * a)).floor();

        if (first * (b - first)) == self.record as f64 {
            first += 1.;
        }
        if (second * (b - second)) == self.record as f64 {
            second -= 1.;
        }

        (second - first) as i64 + 1
    }
}

fn part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().filter(|&s| !s.trim().is_empty()).collect();

    let times = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect::<Vec<i64>>();

    let records = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut races: Vec<RaceInfo> = Vec::new();
    for (&time, &record) in times.iter().zip(records.iter()) {
        races.push(RaceInfo { time, record });
    }
    races.iter().fold(1, |acc, race| acc * race.wins())
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().filter(|&s| !s.trim().is_empty()).collect();

    let time = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let record = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let race_info = RaceInfo { time, record };
    race_info.wins()
}

fn main() {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH).expect("Unable to open file");
    println!("part 1 = {:?}", part1(&input));
    println!("part 2 = {:?}", part2(&input));
}
