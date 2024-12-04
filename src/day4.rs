use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

// Bump this up if it crashes due to index out of range issues.
// Not sure if all puzzle inputs are the same shape, but this lets us
// have stack allocated storage for the grid instead of a heap allocation.
const MAX_GRID_LENGTH: usize = 19_600;

#[derive(Debug)]
struct WordSearch {
    storage: [char; MAX_GRID_LENGTH],
    number_of_columns: usize,
    number_of_rows: usize,
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> WordSearch {
    let mut storage = ['\0'; MAX_GRID_LENGTH];
    let mut index = 0;
    let mut number_of_columns = 0;
    let mut number_of_rows = 1;
    let mut lines = input.trim().lines().map(|line| line.trim());

    // do first line as a special case so it can also set number_of_columns.
    // don't need to worry about that after first line as all rows have same number of columns.
    {
        for character in lines.next().unwrap().chars() {
            number_of_columns += 1;
            storage[index] = character;
            index += 1;
        }
    }

    // process rest of the lines in a similar fashion, but noneed to update number_of_columns.
    for line in lines {
        number_of_rows += 1;
        for character in line.chars() {
            storage[index] = character;
            index += 1;
        }
    }

    WordSearch {
        storage,
        number_of_columns,
        number_of_rows,
    }
}

fn check_bounds(row_index: isize, column_index: isize, input: &WordSearch) -> bool {
    row_index >= 0
        && row_index < input.number_of_rows as isize
        && column_index >= 0
        && column_index < input.number_of_columns as isize
}

fn has_mas_from_location_with_update_func(
    input: &WordSearch,
    mut row_index: isize,
    mut column_index: isize,
    update_func: impl Fn(isize, isize) -> (isize, isize),
) -> bool {
    // check 'M'
    if !check_bounds(row_index, column_index, input) {
        return false;
    }
    if input.storage[row_index as usize * input.number_of_columns + column_index as usize] != 'M' {
        return false;
    }

    // check 'A'
    (row_index, column_index) = update_func(row_index, column_index);
    if !check_bounds(row_index, column_index, input) {
        return false;
    }
    if input.storage[row_index as usize * input.number_of_columns + column_index as usize] != 'A' {
        return false;
    }

    // check 'S'
    (row_index, column_index) = update_func(row_index, column_index);
    if !check_bounds(row_index, column_index, input) {
        return false;
    }
    input.storage[row_index as usize * input.number_of_columns + column_index as usize] == 'S'
}

fn count_xmas_starting_from(input: &WordSearch, row_index: usize, column_index: usize) -> u32 {
    if input.storage[row_index * input.number_of_columns + column_index] == 'X' {
        let mut total = 0;
        let (row_index, column_index) = (row_index as isize, column_index as isize);

        // up
        if has_mas_from_location_with_update_func(
            input,
            row_index - 1,
            column_index,
            |row_index, column_index| (row_index - 1, column_index),
        ) {
            total += 1;
        }

        // down
        if has_mas_from_location_with_update_func(
            input,
            row_index + 1,
            column_index,
            |row_index, column_index| (row_index + 1, column_index),
        ) {
            total += 1;
        }

        // left
        if has_mas_from_location_with_update_func(
            input,
            row_index,
            column_index - 1,
            |row_index, column_index| (row_index, column_index - 1),
        ) {
            total += 1;
        }

        // right
        if has_mas_from_location_with_update_func(
            input,
            row_index,
            column_index + 1,
            |row_index, column_index| (row_index, column_index + 1),
        ) {
            total += 1;
        }

        // up-left
        if has_mas_from_location_with_update_func(
            input,
            row_index - 1,
            column_index - 1,
            |row_index, column_index| (row_index - 1, column_index - 1),
        ) {
            total += 1;
        }

        // up-right
        if has_mas_from_location_with_update_func(
            input,
            row_index - 1,
            column_index + 1,
            |row_index, column_index| (row_index - 1, column_index + 1),
        ) {
            total += 1;
        }

        // down-left
        if has_mas_from_location_with_update_func(
            input,
            row_index + 1,
            column_index - 1,
            |row_index, column_index| (row_index + 1, column_index - 1),
        ) {
            total += 1;
        }

        // down-right
        if has_mas_from_location_with_update_func(
            input,
            row_index + 1,
            column_index + 1,
            |row_index, column_index| (row_index + 1, column_index + 1),
        ) {
            total += 1;
        }

        total
    } else {
        0
    }
}

fn count_x_mas_centered_on(input: &WordSearch, row_index: usize, column_index: usize) -> u32 {
    if input.storage[row_index * input.number_of_columns + column_index] == 'A' {
        let (row_index, column_index) = (row_index as isize, column_index as isize);
        ((has_mas_from_location_with_update_func(
            input,
            row_index - 1,
            column_index - 1,
            |row_index, column_index| (row_index + 1, column_index + 1),
        ) || has_mas_from_location_with_update_func(
            input,
            row_index + 1,
            column_index + 1,
            |row_index, column_index| (row_index - 1, column_index - 1),
        )) && (has_mas_from_location_with_update_func(
            input,
            row_index - 1,
            column_index + 1,
            |row_index, column_index| (row_index + 1, column_index - 1),
        ) || has_mas_from_location_with_update_func(
            input,
            row_index + 1,
            column_index - 1,
            |row_index, column_index| (row_index - 1, column_index + 1),
        ))) as _
    } else {
        0
    }
}

#[aoc(day4, part1)]
fn part1(input: &WordSearch) -> u32 {
    let mut total_words = 0;
    for row_index in 0..input.number_of_rows {
        for column_index in 0..input.number_of_columns {
            total_words += count_xmas_starting_from(input, row_index, column_index);
        }
    }
    total_words
}

#[aoc(day4, part2)]
fn part2(input: &WordSearch) -> u32 {
    let mut total_words = 0;
    for row_index in 0..input.number_of_rows {
        for column_index in 0..input.number_of_columns {
            total_words += count_x_mas_centered_on(input, row_index, column_index);
        }
    }
    total_words
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
		MMMSXXMASM
		MSAMXMSMSA
		AMXSXMAAMM
		MSAMASMSMX
		XMASAMXAMM
		XXAMMXXAMA
		SMSMSASXSS
		SAXAMASAAA
		MAMMMXMMMM
		MXMXAXMASX
		";

    #[test]
    fn test_day_4_part_1() {
        const EXPECTED: u32 = 18;
        let output = part1(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_day_4_part_2() {
        const EXPECTED: u32 = 9;
        let output = part2(&input_generator(INPUT));
        assert_eq!(output, EXPECTED);
    }
}
