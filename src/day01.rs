//! https://adventofcode.com/2023/day/1

use std::collections::HashMap;

use crate::util::readlines;

pub fn trebuchet_pt1(filename: &str) -> Result<u32, &'static str> {
    let mut sum = 0u32;
    let lines = readlines(filename).expect("Error reading file");
    for line in lines {
        if let Ok(text) = line {
            let first_digit = text.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last_digit = text.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            let mut s = String::new();
            s.push(first_digit);
            s.push(last_digit);
            sum += s.parse::<u32>().unwrap();
        }
    }
    Ok(sum)
}

fn get_calibration_value(text: String, pat: &HashMap<&str, char>) -> u32 {
    let first_digit = text.char_indices().find(|(_, c)| c.is_ascii_digit());
    let last_digit = text.char_indices().rev().find(|(_, c)| c.is_ascii_digit());

    let first_eng = pat
        .iter()
        .filter_map(|(s, c)| text.as_str().match_indices(s).next().map(|(i, _)| (i, *c)))
        .min_by_key(|(i, _)| *i);
    let last_eng = pat
        .iter()
        .filter_map(|(s, c)| text.as_str().rmatch_indices(s).next().map(|(i, _)| (i, *c)))
        .max_by_key(|(i, _)| *i);

    let first = [first_digit, first_eng]
        .into_iter()
        .filter_map(|opt| opt)
        .min_by_key(|(i, _)| *i)
        .expect("input is not correct");
    let last = [last_digit, last_eng]
        .into_iter()
        .filter_map(|opt| opt)
        .max_by_key(|(i, _)| *i)
        .expect("input is not correct");

    let mut s = String::new();
    s.push(first.1);
    s.push(last.1);
    s.parse::<u32>().expect("error parsing integer")
}

pub fn trebuchet_pt2(filename: &str) -> Result<u32, &'static str> {
    let pat = [
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .into_iter()
    .collect::<HashMap<&str, char>>();

    let mut sum = 0u32;
    let lines = readlines(filename).expect("Error reading file");
    for line in lines {
        if let Ok(text) = line {
            sum += get_calibration_value(text, &pat);
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_pt1_example() {
        let result = trebuchet_pt1("data/day01.pt1.example");
        assert_eq!(result, Ok(142));
    }

    #[test]
    fn test_day01_pt1_input() {
        let result = trebuchet_pt1("data/day01.input");

        assert_eq!(result, Ok(55123));
    }

    #[test]
    fn test_day01_get_calibration() {
        let pat = [
            ("zero", '0'),
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]
        .into_iter()
        .collect::<HashMap<&str, char>>();
        let s = "1one2three".to_string();
        assert_eq!(get_calibration_value(s, &pat), 13);
    }

    #[test]
    fn test_day01_pt2_example() {
        let result = trebuchet_pt2("data/day01.pt2.example");
        assert_eq!(result, Ok(281));
    }
}
