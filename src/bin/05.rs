fn process_line(line: &str) -> (u32, u32) {
    let mut column_start = 0;
    let mut column_end = 127;
    let mut row_start = 0;
    let mut row_end = 7;

    for i in line.chars() {
        match i {
            'F' => {
                let delta = column_end - column_start;
                column_end = column_start + (delta / 2);
            }
            'B' => {
                let delta = column_end - column_start;
                column_start = column_end - (delta / 2);
            }

            'L' => {
                let delta = row_end - row_start;
                row_end = row_start + (delta / 2);
            }

            'R' => {
                let delta = row_end - row_start;
                row_start = row_end - (delta / 2);
            }

            _ => unreachable!(),
        }
    }

    (column_start, row_start)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(process_line)
            .map(|(r, c)| r * 8 + c)
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut seats = input
        .trim()
        .lines()
        .map(process_line)
        .map(|(r, c)| r * 8 + c)
        .collect::<Vec<_>>();
    seats.sort_unstable();

    Some(seats.windows(2).find(|n| n[0] == n[1] - 2).unwrap()[0] + 1)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(888));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(91));
    }
}
