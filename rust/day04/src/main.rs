const INPUT: &str = include_str!("../input.txt");

pub fn part1() -> u32 {
    // Assumption: numbers have up to 2 digits, so we can only have up to 100 numbers
    let mut winning_numbers = [false; 100];
    let mut winning_numbers_found = 0;
    let mut is_reading_header = true;
    let mut is_reading_winning_numbers = false;
    let mut is_reading_found_numbers = false;
    // Assumption: numbers do not include 0
    let mut current_number = 0;
    let mut total_points = 0;

    INPUT.bytes().for_each(|byte| {
        if (byte == b' ' || byte == b'\n') && current_number != 0 {
            if is_reading_winning_numbers {
                winning_numbers[current_number] = true;
            }
            if is_reading_found_numbers && winning_numbers[current_number] {
                winning_numbers_found += 1;
            }
            current_number = 0;
        }
        if byte == b' ' {
            return;
        }
        if is_reading_header {
            if byte == b':' {
                is_reading_header = false;
                is_reading_winning_numbers = true;
            }
            return;
        }
        if byte == b'|' {
            is_reading_winning_numbers = false;
            is_reading_found_numbers = true;
            return;
        }
        if byte == b'\n' {
            let scratchcard_points = if winning_numbers_found > 0 {
                (2_u32).pow(winning_numbers_found - 1)
            } else {
                0
            };
            total_points += scratchcard_points;

            winning_numbers = [false; 100];
            winning_numbers_found = 0;
            is_reading_found_numbers = false;
            is_reading_header = true;
            return;
        }
        if is_reading_winning_numbers || is_reading_found_numbers {
            current_number = current_number * 10 + (byte - b'0') as usize;
        }
    });

    total_points
}

pub fn part2() -> u32 {
    let mut winning_numbers = [false; 100];

    let scratchcard_points: Vec<usize> = INPUT
        .lines()
        .map(|line| {
            let mut is_reading_winning_numbers = true;
            let mut winning_numbers_found = 0_usize;
            line.split_ascii_whitespace().skip(2).for_each(|chunk| {
                if chunk == "|" {
                    is_reading_winning_numbers = false;
                } else {
                    let number: usize = chunk.parse().unwrap();
                    if is_reading_winning_numbers {
                        winning_numbers[number] = true;
                    } else if winning_numbers[number] {
                        winning_numbers_found += 1;
                    }
                }
            });
            winning_numbers = [false; 100];
            winning_numbers_found
        })
        .collect();

    let mut scratchcard_copies = vec![1; scratchcard_points.len()];

    for (i, &points) in scratchcard_points.iter().enumerate() {
        for n in 1..=points {
            scratchcard_copies[i + n] += scratchcard_copies[i]
        }
    }

    scratchcard_copies.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn works() {
        assert_eq!(part1(), 27454);
        assert_eq!(part2(), 6857330);
    }
}
