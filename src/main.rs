mod days;

use days::run;

use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect(); // env arguments

    // get day
    let day_number: i8 = args
        .get(1)
        .map(|arg| {
            Ok(arg
                .parse::<i8>()
                .map_err(|_| "Day number not provided".to_owned()))
        })
        .unwrap_or(Err("No arguments provided".to_owned()))??;

    // get input data
    let input: String =
        fs::read_to_string("input.txt").map_err(|_| "Couldn't read input.txt".to_owned())?;

    run(day_number, &input)
}
