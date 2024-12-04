use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

type IntegerType = u32;

#[derive(Debug)]
pub enum Command {
    Mul(IntegerType, IntegerType),
    Enable,
    Disable,
}

fn parse_integer(input: &str) -> IResult<&str, IntegerType> {
    map_res(digit1, |digit_str: &str| digit_str.parse())(input)
}

fn parse_mul(input: &str) -> IResult<&str, Command> {
    let (input, (left, right)) = delimited(
        tag("mul("),
        separated_pair(parse_integer, char(','), parse_integer),
        char(')'),
    )(input)?;
    Ok((input, Command::Mul(left, right)))
}

fn parse_enable(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Command::Enable))
}

fn parse_disable(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Command::Disable))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((parse_mul, parse_enable, parse_disable))(input)
}

fn parse_command_with_prefix(input: &str) -> IResult<&str, Command> {
    map(many_till(take(1u8), parse_command), |(_, matched)| matched)(input)
}

fn parse_all_commands(input: &str) -> IResult<&str, Vec<Command>> {
    many0(parse_command_with_prefix)(input)
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Command> {
    let (_, results) = parse_all_commands(input).unwrap();
    results
}

#[aoc(day3, part1)]
pub fn part1(input: &[Command]) -> IntegerType {
    input.into_iter().fold(0, |mut state, command| {
        match command {
            Command::Mul(first, second) => state += first * second,
            _ => {}
        };
        state
    })
}

#[aoc(day3, part2)]
pub fn part2(input: &[Command]) -> IntegerType {
    let mut total = 0;
    let mut enabled = true;
    for command in input {
        match command {
            Command::Enable => enabled = true,
            Command::Disable => enabled = false,
            Command::Mul(left, right) => {
                if enabled {
                    total += left * right
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_part_1() {
        const INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        const EXPECTED: IntegerType = 161;
        let output = part1(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_day_3_part_2() {
        const INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        const EXPECTED: IntegerType = 48;
        let output = part2(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }
}
