use aoc_runner_derive::aoc;
use core::cmp::Ordering;
use std::collections::HashMap;

type IntegerType = u32;

fn parse_page_ordering_rules(input: &str) -> HashMap<(&str, &str), bool> {
    let mut hm = HashMap::new();
    for line in input.lines() {
        let (first, second) = line.trim().split_once("|").unwrap();
        hm.insert((first, second), true);
        hm.insert((second, first), false);
    }
    hm
}

fn parse_updates(input: &str) -> Vec<Vec<&str>> {
    let mut vec = Vec::new();
    for line in input.lines() {
        vec.push(line.trim().split(",").collect());
    }
    vec
}

fn parse_input(input: &str) -> (HashMap<(&str, &str), bool>, Vec<Vec<&str>>) {
    let (rules, updates) = input.trim().split_once("\n\n").unwrap();
    (parse_page_ordering_rules(rules), parse_updates(updates))
}

fn update_is_correct(update: &[&str], rules: &HashMap<(&str, &str), bool>) -> bool {
    for i in 0..update.len() - 1 {
        let ival = update[i];
        for j in i + 1..update.len() {
            if !rules.get(&(ival, update[j])).unwrap_or(&true) {
                return false;
            }
        }
    }
    true
}

fn select_middle_entry<'input>(update: &[&'input str]) -> &'input str {
    update[update.len() / 2]
}

fn parse_middle_entry(update: &[&str]) -> IntegerType {
    select_middle_entry(update).parse().unwrap()
}

fn fix_incorrect_update<'input>(
    update: &[&'input str],
    rules: &HashMap<(&str, &str), bool>,
) -> Vec<&'input str> {
    let mut owned = update.to_owned();
    owned.sort_by(|page_1, page_2| {
        if *rules.get(&(page_1, page_2)).unwrap_or(&true) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    owned
}

#[aoc(day5, part1)]
fn part1(input: &str) -> IntegerType {
    let (rules, updates) = parse_input(input);
    updates
        .into_iter()
        .filter_map(|update| {
            if update_is_correct(&update, &rules) {
                Some(parse_middle_entry(&update))
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);
    updates
        .into_iter()
        .filter_map(|update| {
            if !update_is_correct(&update, &rules) {
                Some(parse_middle_entry(&fix_incorrect_update(&update, &rules)))
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
    47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47
    ";

    #[test]
    fn test_day_5_part_1() {
        const EXPECTED: IntegerType = 143;
        let output = part1(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_day_5_part_2() {
        const EXPECTED: IntegerType = 123;
        let output = part2(INPUT);
        assert_eq!(output, EXPECTED);
    }
}
