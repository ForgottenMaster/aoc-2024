use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use rayon::prelude::*;

// Tweak these if there's index out of range issues.
const MAX_NUMBER_OF_EQUATIONS: usize = 850;
const MAX_TERMS_PER_EQUATION: usize = 12;

// Tweak this if there's issues with incorrect answers, etc. due to overflow.
type IntegerType = u64;

struct Equations {
    equations: [Equation; MAX_NUMBER_OF_EQUATIONS],
    len: usize,
}

#[derive(Clone, Copy)]
struct Equation {
    result: IntegerType,
    terms: [IntegerType; MAX_TERMS_PER_EQUATION],
    len: usize,
}

impl Default for Equation {
    fn default() -> Self {
        Self {
            result: 0,
            terms: [0; MAX_TERMS_PER_EQUATION],
            len: 0,
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Equations {
    let mut equations_array = [Equation::default(); MAX_NUMBER_OF_EQUATIONS];
    let mut equations_len = 0;
    for line in input.trim().lines() {
        let (result, terms) = line.trim().split_once(": ").unwrap();
        let result = result.parse().unwrap();
        let mut terms_array = [0; MAX_TERMS_PER_EQUATION];
        let mut terms_len = 0;
        for term in terms.split(" ") {
            terms_array[terms_len] = term.parse().unwrap();
            terms_len += 1;
        }
        equations_array[equations_len] = Equation {
            result,
            terms: terms_array,
            len: terms_len,
        };
        equations_len += 1;
    }
    Equations {
        equations: equations_array,
        len: equations_len,
    }
}

const fn concat(a: IntegerType, b: IntegerType) -> IntegerType {
    a * (10 as IntegerType).pow(b.ilog10() + 1) + b
}

fn equation_satisfies_part_1(
    result: IntegerType,
    current: IntegerType,
    remaining_terms: &[IntegerType],
) -> bool {
    if remaining_terms.len() == 0 {
        result == current
    } else {
        let (next_term, remaining_terms) = remaining_terms.split_at(1);
        let next_term = next_term[0];
        equation_satisfies_part_1(result, current + next_term, remaining_terms)
            || equation_satisfies_part_1(result, current * next_term, remaining_terms)
    }
}

#[aoc(day7, part1)]
fn part1(input: &Equations) -> IntegerType {
    input.equations[..input.len]
        .par_iter()
        .filter_map(|equation| {
            if equation_satisfies_part_1(
                equation.result,
                equation.terms[0],
                &equation.terms[1..equation.len],
            ) {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum()
}

fn equation_satisfies_part_2(
    result: IntegerType,
    current: IntegerType,
    remaining_terms: &[IntegerType],
) -> bool {
    if remaining_terms.len() == 0 {
        result == current
    } else {
        let (next_term, remaining_terms) = remaining_terms.split_at(1);
        let next_term = next_term[0];
        equation_satisfies_part_2(result, current + next_term, remaining_terms)
            || equation_satisfies_part_2(result, current * next_term, remaining_terms)
            || equation_satisfies_part_2(result, concat(current, next_term), remaining_terms)
    }
}

#[aoc(day7, part2)]
fn part2(input: &Equations) -> IntegerType {
    input.equations[..input.len]
        .par_iter()
        .filter_map(|equation| {
            if equation_satisfies_part_2(
                equation.result,
                equation.terms[0],
                &equation.terms[1..equation.len],
            ) {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
    ";

    #[test]
    fn test_day_7_part_1() {
        const EXPECTED: IntegerType = 3749;
        let output = part1(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_day_7_part_2() {
        const EXPECTED: IntegerType = 11387;
        let output = part2(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }
}
