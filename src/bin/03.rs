fn calculate(text: &str, right: u32) -> u32 {
    let mut i = 0;

    text.lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            let ans = if chars[i] == '#' { 1 } else { 0 };
            i += right as usize;
            i %= chars.len();
            ans
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(calculate(input, 3))
}

pub fn part_two(input: &str) -> Option<u32> {
    let down_two: Vec<&str> = input.lines().step_by(2).collect();

    let mut i = 0;

    let result: u32 = down_two
        .iter()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            let ans = if chars[i] == '#' { 1 } else { 0 };
            i += 1;
            i %= chars.len();
            ans
        })
        .sum();

    Some(
        calculate(input, 1)
            * calculate(input, 3)
            * calculate(input, 5)
            * calculate(input, 7)
            * result,
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(336));
    }
}
