mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
// mod day16;
mod day17;

pub fn run(day: i8, input: &String) -> Result<(), String> {
    match day {
        1 => day01::run(input),
        2 => day02::run(input),
        3 => day03::run(input),
        4 => day04::run(input),
        5 => day05::run(input),
        6 => day06::run(input),
        7 => day07::run(input),
        8 => day08::run(input), // TODO: Part 2
        9 => day09::run(input),
        10 => day10::run(input),
        11 => day11::run(input),
        12 => day12::run(input), // TODO: Part 2
        13 => day13::run(input),
        14 => day14::run(input),
        15 => day15::run(input),
        // TODO: 16
        17 => day17::run(input), // TODO: Part 2
        d => Err(format!("Invalid day `{d}`"))?,
    }
    Ok(())
}
