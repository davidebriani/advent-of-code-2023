const INPUT: &str = include_str!("../input.txt");

pub fn part1() -> u32 {
    let mut sections = INPUT.split("\n\n");

    let mut seeds: Vec<u64> = sections
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|chunk| chunk.parse().unwrap())
        .collect();

    let maps: Vec<Vec<Vec<u64>>> = sections
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    line.split(' ')
                        .map(|digits| digits.parse::<u64>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    for seed in seeds.iter_mut() {
        for map in maps.iter() {
            for instruction in map {
                let dest = instruction[0];
                let start = instruction[1];
                let length = instruction[2];
                let end = start + length;
                if start <= *seed && *seed < end {
                    *seed = *seed - start + dest;
                    break;
                }
            }
        }
    }

    let &min = seeds.iter().min().unwrap();
    min as u32
}

pub fn part2() -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn works() {
        assert_eq!(part1(), 836040384);
        assert_eq!(part2(), 0);
    }
}
