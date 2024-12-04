mod day1;
mod day2;
mod day3;
mod day4;

pub fn run(day: i8, input: &String) -> Result<(), String> {
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        4 => day4::run(input),
        d => Err(format!("Invalid day `{d}`"))?,
    }
    Ok(())
}
