use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

pub fn part1() -> u32 {
    let row_length = INPUT.lines().next().unwrap().len() + 1; // account for '\n'
    let input_length = INPUT.len() + 1; // account for missing last '\n' to have a round total
    let row_count = input_length / row_length;

    let mut numbers: Vec<u32> = vec![0];
    let mut symbol_positions: Vec<(usize, usize)> = vec![];
    let mut number_index_grid = vec![0; input_length];

    let mut current_number_index = 1;
    let mut current_number = 0;

    INPUT.bytes().enumerate().for_each(|(i, byte)| {
        let row = i / row_length;
        let column = i % row_length;
        // Parse numbers
        if byte.is_ascii_digit() {
            current_number = (current_number * 10) + (byte - b'0') as u32;
            number_index_grid[row * row_length + column] = current_number_index;
        } else {
            if current_number != 0 {
                numbers.push(current_number);
                current_number = 0;
                current_number_index += 1;
            }

            // Parse symbols
            if byte != b'.' && byte != b'\n' {
                symbol_positions.push((row, column));
            }
        }
    });

    symbol_positions
        .iter()
        .flat_map(|&(row, column)| {
            let start_row = if row > 0 { row - 1 } else { row };
            let start_column = if column > 0 { column - 1 } else { column };
            let end_row = if row < row_count - 1 { row + 1 } else { row };
            let end_column = if column < row_length - 1 {
                column + 1
            } else {
                column
            };
            let number_index_grid = &number_index_grid;
            (start_row..=end_row).flat_map(move |r| {
                (start_column..=end_column).map(move |c| number_index_grid[r * row_length + c])
            })
        })
        .unique()
        .map(|number_index| numbers[number_index])
        .sum()
}

pub fn part2() -> u32 {
    let row_length = INPUT.lines().next().unwrap().len() + 1; // account for '\n'
    let input_length = INPUT.len() + 1; // account for missing last '\n' to have a round total
    let row_count = input_length / row_length;

    let mut numbers: Vec<u32> = vec![0];
    let mut gear_positions: Vec<(usize, usize)> = vec![];
    let mut number_index_grid = vec![0; input_length];

    let mut current_number_index = 1;
    let mut current_number = 0;

    INPUT.bytes().enumerate().for_each(|(i, byte)| {
        let row = i / row_length;
        let column = i % row_length;
        // Parse numbers
        if byte.is_ascii_digit() {
            current_number = (current_number * 10) + (byte - b'0') as u32;
            number_index_grid[row * row_length + column] = current_number_index;
        } else {
            if current_number != 0 {
                numbers.push(current_number);
                current_number = 0;
                current_number_index += 1;
            }

            // Parse gears
            if byte == b'*' {
                gear_positions.push((row, column));
            }
        }
    });

    gear_positions
        .iter()
        .map(|&(row, column)| {
            let start_row = if row > 0 { row - 1 } else { row };
            let start_column = if column > 0 { column - 1 } else { column };
            let end_row = if row < row_count - 1 { row + 1 } else { row };
            let end_column = if column < row_length - 1 {
                column + 1
            } else {
                column
            };
            let mut number_count = 0;
            let mut first_number = 0;
            let mut second_number = 0;
            for r in start_row..=end_row {
                for c in start_column..=end_column {
                    let number_index = number_index_grid[r * row_length + c];
                    let number = numbers[number_index];
                    if number != 0 {
                        if number_count == 0 {
                            number_count += 1;
                            first_number = number;
                        }
                        if number_count == 1 && number != first_number {
                            number_count += 1;
                            second_number = number;
                        }
                        if number_count > 1 && number != first_number && number != second_number {
                            number_count += 1;
                        }
                    }
                }
            }
            if number_count == 2 {
                first_number * second_number
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn works() {
        assert_eq!(part1(), 530495);
        assert_eq!(part2(), 80253814);
    }
}
