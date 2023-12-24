use std::fs;


fn part1(input: &str) {
    todo!()
}

fn main() {
    const FILE_PATH: &str = "sample.txt";
    let input = fs::read_to_string(FILE_PATH).expect("Unable to open file");
    println!("part 1 = {:?}", part1(&input));
}
