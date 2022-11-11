use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r#"(?P<low>\d+)-(?P<high>\d+) (?P<letter>\w): (?P<password>\w+)"#).unwrap();
    let mut ans = 0;
    for cap in re.captures_iter(input) {
        let low: i32 = cap.name("low").unwrap().as_str().parse().unwrap();
        let high: i32 = cap.name("high").unwrap().as_str().parse().unwrap();
        let letter = cap.name("letter").unwrap().as_str();
        let password = cap.name("password").unwrap().as_str();

        let mut count = 0;
        for c in password.chars() {
            if c == letter.chars().next().unwrap() {
                count += 1;
            }
        }

        ans += if (low <= count) && (count <= high) {
            1
        } else {
            0
        };
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r#"(?P<low>\d+)-(?P<high>\d+) (?P<letter>\w): (?P<password>\w+)"#).unwrap();
    let mut ans = 0;
    for cap in re.captures_iter(input) {
        let low: i32 = cap.name("low").unwrap().as_str().parse().unwrap();
        let high: i32 = cap.name("high").unwrap().as_str().parse().unwrap();
        let letter = cap.name("letter").unwrap().as_str();
        let password = cap.name("password").unwrap().as_str();

        let low_char = password.chars().collect::<Vec<_>>()[low as usize - 1];
        let high_char = password.chars().collect::<Vec<_>>()[high as usize - 1];
        let letter_char = letter.chars().next().unwrap();

        ans += if (low_char == letter_char && high_char != letter_char)
            || (low_char != letter_char && high_char == letter_char)
        {
            1
        } else {
            0
        }
    }
    Some(ans)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(1));
    }
}
