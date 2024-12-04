use regex::Regex;

pub fn run(input: &String) {
    println!("--- Day 3: Mull It Over ---");

    // spaghetti
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)()()|don't\(\)()()").unwrap();

    let mut accumulator_1: u64 = 0;
    let mut accumulator_2: u64 = 0;
    let mut enabled = true;
    re.captures_iter(input).for_each(|c| {
        let (m, [n1, n2]) = c.extract::<2>();
        if matches!(m, "do()") {
            enabled = true;
        } else if matches!(m, "don't()") {
            enabled = false;
        } else {
            let n1 = n1.parse::<u64>().unwrap();
            let n2 = n2.parse::<u64>().unwrap();
            accumulator_1 += n1 * n2;
            accumulator_2 += n1 * n2 * enabled as u64;
        }
    });

    println!("Star 1: {accumulator_1}");
    println!("Star 2: {accumulator_2}");
}