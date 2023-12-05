const INPUT: &str = include_str!("../input.txt");

pub fn part1() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            first * 10 + last
        })
        .sum()
}

pub fn part2() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .map(|i| &line[i..line.len()])
                .find_map(parse_digit)
                .unwrap();
            let last = (0..line.len())
                .map(|i| &line[(line.len() - 1 - i)..line.len()])
                .find_map(parse_digit)
                .unwrap();
            first * 10 + last
        })
        .sum()
}

fn parse_digit(subtring: &str) -> Option<u32> {
    parse_numeric_digit(subtring).or_else(|| parse_literal_digit(subtring))
}

fn parse_numeric_digit(subtring: &str) -> Option<u32> {
    subtring.chars().next().and_then(|c| c.to_digit(10))
}

fn parse_literal_digit(subtring: &str) -> Option<u32> {
    [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .find_map(|(index, number)| {
        if subtring.starts_with(number) {
            Some((index + 1) as u32)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn works() {
        assert_eq!(part1(), 55002);
        assert_eq!(part2(), 55093);
    }
}
