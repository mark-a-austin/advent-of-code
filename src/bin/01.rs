use std::num::ParseIntError;

advent_of_code::solution!(1);

// ! means macros -> similar to C macros

/**
 * The calibration document contains lines of text embedded is two numbers
 * that need to be combined together to produce the calibration value.
 *
 * These then need to be summed together to produce the final calibration value.
 */
pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for line in lines {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let calibration_value = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        let calibration_value: u32 = calibration_value.parse().expect("Please be int");
        sum += calibration_value;
    }
    return Some(sum);
}

fn get_spelled_numbers(input: &str) -> Vec<&str> {
    let pairs = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut words = vec![];
    for i in 0..input.len() {
        let sliced = &input[i..];
        for (word, j) in pairs {
            if sliced.starts_with(word) || sliced.starts_with(j) {
                words.push(j);
                break;
            }
        }
    }
    return words;
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for line in lines {
        let words = get_spelled_numbers(line);
        let calibration_value = format!("{}{}", words.first().unwrap(), words.last().unwrap());
        let calibration_value: u32 = calibration_value.parse().expect("None integer found!");
        sum += calibration_value;
    }
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
