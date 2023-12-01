use std::fs;
use std::io::Error;

pub fn part1(path_to_file: &str) -> Result<Option<u32>, Error> {
    let mut numbers: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    let file_contents = fs::read_to_string(path_to_file).expect("Could not read file");

    for line in file_contents.lines() {
        let mut numbers_in_line: Vec<char> = Vec::new();

        for char in line.chars() {
            if char.is_numeric() {
                numbers_in_line.push(char);
            }
        }

        let number_as_char = format!("{}{}", numbers_in_line[0], numbers_in_line[numbers_in_line.len() - 1]);
        numbers.push((&number_as_char as &str).parse().unwrap())
    }

    for number in numbers {
        sum = sum + number;
    }

    Ok(Some(sum))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run() {
        let result = super::part1("src/days/day1/part1_test.txt").expect("Could not calculate sum");
        assert_eq!(result, Some(142u32))
    }
}