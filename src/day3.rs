use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

type IntegerType = u32;

fn parse_integer(input: &str) -> IResult<&str, IntegerType> {
    map_res(digit1, |digit_str: &str| digit_str.parse())(input)
}

fn parse_mul(input: &str) -> IResult<&str, (IntegerType, IntegerType)> {
    delimited(
        tag("mul("),
        separated_pair(parse_integer, char(','), parse_integer),
        char(')'),
    )(input)
}

fn parse_mul_with_prefix(input: &str) -> IResult<&str, (IntegerType, IntegerType)> {
    map(many_till(take(1u8), parse_mul), |(_, matched)| matched)(input)
}

fn parse_all_mul(input: &str) -> IResult<&str, Vec<(IntegerType, IntegerType)>> {
    many0(parse_mul_with_prefix)(input)
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(IntegerType, IntegerType)> {
    let (_, results) = parse_all_mul(input).unwrap();
    results
}

#[aoc(day3, part1)]
pub fn part1(input: &[(IntegerType, IntegerType)]) -> IntegerType {
    input.into_iter().fold(0, |mut state, (first, second)| {
        state += first * second;
        state
    })
}

#[aoc(day3, part2)]
pub fn part2(input: &[(IntegerType, IntegerType)]) -> IntegerType {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part_1() {
        const EXPECTED: IntegerType = 161;
        let output = part1(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }
}
