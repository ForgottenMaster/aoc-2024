use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let lines = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());
    let count = lines.clone().count();
    let (mut left, mut right) = lines.fold(
        (Vec::<u32>::with_capacity(count), Vec::with_capacity(count)),
        |(mut left, mut right), line| {
            let mut line = line.split_whitespace();
            let (first, second) = (line.next().unwrap(), line.next().unwrap());
            left.push(first.parse().unwrap());
            right.push(second.parse().unwrap());
            (left, right)
        },
    );
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(left, right)| {
            if left < right {
                right - left
            } else {
                left - right
            }
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let lines = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());
    let count = lines.clone().count();
    let (left, right) = lines.fold(
        (
            Vec::<u32>::with_capacity(count),
            HashMap::<u32, u32>::with_capacity(count),
        ),
        |(mut left, mut right), line| {
            let mut line = line.split_whitespace();
            let (first, second) = (line.next().unwrap(), line.next().unwrap());
            left.push(first.parse().unwrap());
            *right.entry(second.parse().unwrap()).or_insert(0) += 1;
            (left, right)
        },
    );
    left.into_iter()
        .map(|num| num * *right.get(&num).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    ";

    #[test]
    fn test_day_1_part_1() {
        const EXPECTED: u32 = 11;
        let output = part1(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_day_1_part_2() {
        const EXPECTED: u32 = 31;
        let output = part2(INPUT);
        assert_eq!(output, EXPECTED);
    }
}
