mod days;

fn main() {
    let day1_part1_result = days::day1::parts::part1("input/day1/part1.txt").expect("Could not calculate ");
    println!("Answer of Day 1 - Part 1: {}", day1_part1_result);
    let day1_part2_result = days::day1::parts::part2("input/day1/part1.txt").expect("Could not calculate ");
    println!("Answer of Day 1 - Part 2: {}", day1_part2_result);
}
