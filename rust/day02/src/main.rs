use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

struct RedGreenBlue(u32, u32, u32);

pub fn part1() -> u32 {
    minimum_colors_for_each_game()
        .enumerate()
        .flat_map(|(index, RedGreenBlue(r, g, b))| {
            if r > 12 || g > 13 || b > 14 {
                None
            } else {
                Some((index + 1) as u32)
            }
        })
        .sum()
}

pub fn part2() -> u32 {
    minimum_colors_for_each_game()
        .map(|RedGreenBlue(r, g, b)| r * g * b)
        .sum()
}

fn minimum_colors_for_each_game() -> impl Iterator<Item = RedGreenBlue> {
    INPUT.lines().map(|line| {
        line.split_ascii_whitespace()
            .skip(2)
            .chunks(2)
            .into_iter()
            .fold(
                RedGreenBlue(0, 0, 0),
                |RedGreenBlue(r, g, b), mut line_chunk| {
                    let count: u32 = line_chunk.next().unwrap().parse().unwrap();
                    let color = line_chunk.last().unwrap();
                    match color.bytes().next().unwrap() {
                        b'r' => RedGreenBlue(r.max(count), g, b),
                        b'g' => RedGreenBlue(r, g.max(count), b),
                        b'b' => RedGreenBlue(r, g, b.max(count)),
                        _ => unreachable!(),
                    }
                },
            )
    })
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn works() {
        assert_eq!(part1(), 2685);
        assert_eq!(part2(), 83707);
    }
}
