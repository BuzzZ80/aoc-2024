mod day1;
mod day2;

pub fn run(day: i8, input: &String) -> Result<(), String> {
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        d => Err(format!("Invalid day `{d}`"))?,
    }
    Ok(())
}
