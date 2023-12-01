use std::collections::HashMap;
use std::fs;
use std::io::Error;

pub fn part1(path_to_file: &str) -> Result<u32, Error> {
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

        if !numbers_in_line.is_empty() {
            let number_as_char = format!("{}{}", numbers_in_line[0], numbers_in_line[numbers_in_line.len() - 1]);
            numbers.push((&number_as_char as &str).parse().unwrap())
        }
    }

    for number in numbers {
        sum += number;
    }

    Ok(sum)
}

pub fn part2(path_to_file: &str) -> Result<u32, Error> {
    let mut numbers: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    let file_contents = fs::read_to_string(path_to_file).expect("Could not read file");

    for line in file_contents.lines() {
        let mut numbers_in_line: Vec<char> = Vec::new();
        let mut chars_in_row = String::new();

        for char in line.chars() {
            if char.is_numeric() {
                extract_number_from_string(&mut numbers_in_line, &chars_in_row);

                chars_in_row = String::new();
                numbers_in_line.push(char);
            } else {
                chars_in_row.push(char);
            }
        }
        extract_number_from_string(&mut numbers_in_line, &chars_in_row);

        if !numbers_in_line.is_empty() {
            let number_as_char = format!("{}{}", numbers_in_line[0], numbers_in_line[numbers_in_line.len() - 1]);
            numbers.push((&number_as_char as &str).parse().unwrap())
        }
    }

    for number in numbers {
        sum += number;
    }

    Ok(sum)
}

fn extract_number_from_string(numbers_in_line: &mut Vec<char>, chars_in_row: &str) {
    let mut found_numbers: HashMap<usize, char> = HashMap::new();
    let mut lowest = 0;
    let mut highest = 0;
    let mut numbers_as_word: HashMap<&str, char> = HashMap::new();
    numbers_as_word.insert("one", '1');
    numbers_as_word.insert("two", '2');
    numbers_as_word.insert("three", '3');
    numbers_as_word.insert("four", '4');
    numbers_as_word.insert("five", '5');
    numbers_as_word.insert("six", '6');
    numbers_as_word.insert("seven", '7');
    numbers_as_word.insert("eight", '8');
    numbers_as_word.insert("nine", '9');

    let mut iterations: u32 = 0;
    for (word, char) in numbers_as_word.into_iter() {
        if chars_in_row.contains(word) {
            let idx_low = chars_in_row.find(word).unwrap();
            let idx_high = chars_in_row.rfind(word).unwrap();

            if iterations > 0 {
                if idx_high > highest {
                    highest = idx_high;
                }

                if idx_low < lowest {
                    lowest = idx_low;
                }
            } else {
                highest = idx_high;
                lowest = idx_low;
            }

            found_numbers.insert(idx_low, char);
            found_numbers.insert(idx_high, char);
            iterations += 1;
        }
    }

    if !found_numbers.is_empty() {
        if numbers_in_line.is_empty() {
            numbers_in_line.push(*found_numbers.get(&lowest).unwrap());
        }

        numbers_in_line.push(*found_numbers.get(&highest).unwrap());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let result = super::part1("src/days/day1/part1_test.txt").expect("Could not calculate sum");
        assert_eq!(result, 142u32)
    }

    #[test]
    fn test_part2() {
        let result = super::part2("src/days/day1/part2_test.txt").expect("Could not calculate sum");
        assert_eq!(result, 281u32)
    }
}