use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use rayon::prelude::*;
use std::cell::{OnceCell, RefCell};
use std::collections::HashSet;

// Bump this up if it crashes due to index out of range issues.
// Not sure if all puzzle inputs are the same shape, but this lets us
// have stack allocated storage for the grid instead of a heap allocation.
const MAX_GRID_LENGTH: usize = 19_600;

#[derive(Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Map {
    obstacles: [bool; MAX_GRID_LENGTH],
    number_of_columns: usize,
    number_of_rows: usize,
    guard: ((usize, usize), Direction),
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Map {
    let mut obstacles = [false; MAX_GRID_LENGTH];
    let mut index = 0;
    let mut number_of_columns = 0;
    let mut number_of_rows = 1;
    let mut lines = input.trim().lines().map(|line| line.trim());
    let mut guard = (0, Direction::Up);

    // do first line as a special case so it can also set number_of_columns.
    // don't need to worry about that after first line as all rows have same number of columns.
    {
        for character in lines.next().unwrap().chars() {
            number_of_columns += 1;
            obstacles[index] = character == '#';
            match character {
                '#' => obstacles[index] = true,
                '>' => guard = (index, Direction::Right),
                '<' => guard = (index, Direction::Left),
                '^' => guard = (index, Direction::Up),
                'v' => guard = (index, Direction::Down),
                _ => {}
            }
            index += 1;
        }
    }

    // process rest of the lines in a similar fashion, but noneed to update number_of_columns.
    for line in lines {
        number_of_rows += 1;
        for character in line.chars() {
            match character {
                '#' => obstacles[index] = true,
                '>' => guard = (index, Direction::Right),
                '<' => guard = (index, Direction::Left),
                '^' => guard = (index, Direction::Up),
                'v' => guard = (index, Direction::Down),
                _ => {}
            }
            index += 1;
        }
    }
    let (guard_index, guard_direction) = guard;
    let (row, column) = (
        guard_index / number_of_columns,
        guard_index % number_of_columns,
    );
    let guard = ((row, column), guard_direction);

    Map {
        obstacles,
        number_of_columns,
        number_of_rows,
        guard,
    }
}

fn move_guard(guard: &mut ((usize, usize), Direction), input: &Map) -> bool {
    let ((row, column), direction) = guard;
    match direction {
        Direction::Left => {
            if *column == 0 {
                false
            } else {
                let new_column = *column - 1;
                let index = *row * input.number_of_columns + new_column;
                if input.obstacles[index] {
                    *direction = Direction::Up;
                } else {
                    *column = new_column;
                }
                true
            }
        }
        Direction::Right => {
            if *column == input.number_of_columns - 1 {
                false
            } else {
                let new_column = *column + 1;
                let index = *row * input.number_of_columns + new_column;
                if input.obstacles[index] {
                    *direction = Direction::Down;
                } else {
                    *column = new_column;
                }
                true
            }
        }
        Direction::Up => {
            if *row == 0 {
                false
            } else {
                let new_row = *row - 1;
                let index = new_row * input.number_of_columns + *column;
                if input.obstacles[index] {
                    *direction = Direction::Right;
                } else {
                    *row = new_row;
                }
                true
            }
        }
        Direction::Down => {
            if *row == input.number_of_rows - 1 {
                false
            } else {
                let new_row = *row + 1;
                let index = new_row * input.number_of_columns + *column;
                if input.obstacles[index] {
                    *direction = Direction::Left;
                } else {
                    *row = new_row;
                }
                true
            }
        }
    }
}

fn get_distinct_positions(input: &Map) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut guard = input.guard.clone();
    visited.insert(guard.0);
    while move_guard(&mut guard, input) {
        visited.insert(guard.0);
    }
    visited
}

fn has_cycle(input: &Map, visited: &mut HashSet<((usize, usize), Direction)>) -> bool {
    visited.clear();
    let mut guard = input.guard.clone();
    visited.insert(guard.clone());
    while move_guard(&mut guard, input) {
        if !visited.insert(guard.clone()) {
            return true;
        }
    }
    false
}

#[aoc(day6, part1)]
fn part1(input: &Map) -> usize {
    get_distinct_positions(input).len()
}

#[aoc(day6, part2)]
fn part2(input: &Map) -> usize {
    thread_local! {
        static VISITED: RefCell<HashSet<((usize, usize), Direction)>> = RefCell::new(HashSet::new());
        static INPUT: RefCell<OnceCell<Map>> = RefCell::new(OnceCell::new());
    }

    get_distinct_positions(&input)
        .par_iter()
        .map(|position| {
            if *position != input.guard.0 {
                INPUT.with_borrow_mut(|input_mut| {
                    let input_mut_ref = input_mut.get_mut();
                    if input_mut_ref.is_none() {
                        let _ = input_mut.set(input.clone());
                    }
                    let input = input_mut.get_mut().unwrap();
                    let (row, column) = *position;
                    let index = row * input.number_of_columns + column;
                    input.obstacles[index] = true; // place obstacle.
                    let return_value =
                        VISITED.with_borrow_mut(
                            |visited| if has_cycle(&input, visited) { 1 } else { 0 },
                        );
                    input.obstacles[index] = false; // unplace obstacle.
                    return_value
                })
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
    ";

    #[test]
    fn test_day_6_part_1() {
        const EXPECTED: usize = 41;
        let output = part1(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_day_6_part_2() {
        const EXPECTED: usize = 6;
        let output = part2(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }
}
