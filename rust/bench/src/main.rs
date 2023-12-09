use took::{Timer, Took};

const RUNS: usize = 1000;

fn main() {
    println!("Benchmarking all days with {} runs...", RUNS);

    let times: Vec<_> = jobs()
        .iter()
        .map(|(job, name)| {
            let min_duration = (0..RUNS)
                .map(|_| {
                    let took = Timer::new();
                    job();
                    took.took().into_std()
                })
                .min()
                .unwrap();
            (name, min_duration)
        })
        .collect();

    for (name, duration) in &times {
        Took::from_std(*duration).describe(name);
    }

    Took::from_std(times.into_iter().map(|(_, t)| t).sum()).describe("everything");
}

fn jobs() -> &'static [(fn() -> u32, &'static str)] {
    &[
        (day01::part1, "day01_part1"),
        (day01::part2, "day01_part2"),
        (day02::part1, "day02_part1"),
        (day02::part2, "day02_part2"),
        (day03::part1, "day03_part1"),
        (day03::part2, "day03_part2"),
        (day04::part1, "day04_part1"),
        (day04::part2, "day04_part2"),
    ]
}
