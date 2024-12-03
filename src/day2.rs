use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(mut input: impl Iterator<Item = i8>) -> bool {
    let mut total_signum = 0;
    let mut count = 0;
    let mut prev = input.next().unwrap();
    for next in input {
        let diff = next - prev;
        let absdiff = diff.abs();
        if absdiff < 1 || absdiff > 3 {
            return false;
        }
        total_signum += diff.signum();
        count += 1;
        prev = next;
    }
    total_signum.abs() == count
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<i8>]) -> usize {
    input
        .into_iter()
        .filter(|report| is_safe(report.into_iter().copied()))
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<i8>]) -> usize {
    input
        .into_iter()
        .filter(|report| {
            if is_safe(report.into_iter().copied()) {
                return true;
            }
            (0..report.len()).any(|index_to_remove| {
                is_safe(
                    report
                        .into_iter()
                        .copied()
                        .enumerate()
                        .filter_map(|(index, element)| {
                            if index == index_to_remove {
                                None
                            } else {
                                Some(element)
                            }
                        }),
                )
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    ";

    #[test]
    fn test_day_2_part_1() {
        const EXPECTED: usize = 2;
        let output = part1(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_day_2_part_2() {
        const EXPECTED: usize = 4;
        let output = part2(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }
}
