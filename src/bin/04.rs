use std::collections::{HashMap, HashSet};

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub fn part_one(input: &str) -> Option<u32> {
    let answer: usize = input
        .split("\n\n")
        .map(|fields| {
            fields
                .split_ascii_whitespace()
                .map(|field| field.split(':').next().unwrap())
                .collect::<HashSet<_>>()
        })
        .filter(|passport| REQUIRED_FIELDS.iter().all(|item| passport.contains(item)))
        .count();
    Some(answer as u32)
}

fn validate_passport(field: &str, value: &str) -> bool {
    match field {
        "byr" => value.parse::<usize>().unwrap().wrapping_sub(1920) <= 82,
        "iyr" => value.parse::<usize>().unwrap().wrapping_sub(2010) <= 10,
        "eyr" => value.parse::<usize>().unwrap().wrapping_sub(2020) <= 10,
        "hgt" => {
            if value.ends_with("cm") && value.len() == 5 {
                value[0..3].parse::<usize>().unwrap().wrapping_sub(150) <= 43
            } else if value.ends_with("in") && value.len() == 4 {
                value[0..2].parse::<usize>().unwrap().wrapping_sub(59) <= 27
            } else {
                false
            }
        }

        "hcl" => value.len() == 7,
        "ecl" => EYE_COLORS.iter().any(|v| v == &value),
        "pid" => value.len() == 9,
        "cid" => true,
        _ => unreachable!(),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer: usize = input
        .split("\n\n")
        .map(|fields| {
            fields
                .split_ascii_whitespace()
                .map(|field| field.split_once(':').unwrap())
                .collect::<HashMap<_, _>>()
        })
        .filter(|passport| {
            REQUIRED_FIELDS
                .iter()
                .all(|item| passport.contains_key(item))
        })
        .filter(|passport| passport.iter().all(|(f, v)| validate_passport(f, v)))
        .count();
    Some(answer as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(2));
    }
}
