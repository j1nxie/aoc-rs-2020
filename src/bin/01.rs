use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = input
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();

    let mut ans = 0;

    for num in numbers.iter() {
        if numbers.contains(&(2020 - num)) {
            ans = num * (2020 - num);
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let numbers = input
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<HashSet<_>>();

    let mut ans = 0;

    let numbers_vector: Vec<&i64> = numbers.iter().collect();
    for i in 0..numbers_vector.len() {
        for j in 0..numbers_vector.len() {
            let third: i64 = 2020 - numbers_vector[i] - numbers_vector[j];
            if numbers.contains(&third) {
                ans = third * numbers_vector[i] * numbers_vector[j];
            }
        }
    }

    Some(ans as u64)
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(514579));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(241861950));
    }
}
