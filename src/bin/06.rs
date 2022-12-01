use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let groups: Vec<&str> = input.split("\n\n").collect();
    Some(
        groups
            .iter()
            .map(|&g| {
                g.chars()
                    .filter(|&c| c != '\n')
                    .collect::<HashSet<_>>()
                    .len()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let groups: Vec<&str> = input.split("\n\n").collect();
    Some(
        groups
            .iter()
            .map(|&g| {
                g.lines()
                    .map(|l| l.chars().filter(|&c| c != '\n').collect::<HashSet<_>>())
                    .fold(('a'..='z').collect::<HashSet<_>>(), |acc, s| {
                        acc.intersection(&s).cloned().collect()
                    })
                    .len()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(6));
    }
}
