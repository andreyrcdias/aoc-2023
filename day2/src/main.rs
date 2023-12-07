use std::fs::read_to_string;

fn part1(input: &str) -> u32 {
    const MAX_COLORS: [u32; 3] = [12, 13, 14];

    input
        .lines()
        .map(|line| {
            let game_id = line[5..line.find(':').unwrap()].parse::<u32>().unwrap();
            let mut game_sets = line[line.find(':').unwrap() + 2..line.len()].split("; ");

            let valid = game_sets.all(|game_set| {
                game_set.split(", ").all(|color| {
                    let mut color_parts = color.split_whitespace();
                    let color_qty = color_parts.next().unwrap().parse::<u32>().unwrap();
                    let color_value = color_parts.next().unwrap();
                    match color_value {
                        "red" => color_qty <= MAX_COLORS[0],
                        "green" => color_qty <= MAX_COLORS[1],
                        "blue" => color_qty <= MAX_COLORS[2],
                        _ => true,
                    }
                })
            });
            if valid {
                game_id
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    const FILE_PATH: &'static str = "input.txt";
    let input = read_to_string(FILE_PATH).expect(&format!("Unable to read {}", FILE_PATH));
    println!("part 1 = {:?}", part1(&input));
}
