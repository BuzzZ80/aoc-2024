use regex::Regex;

pub fn run(input: &String) {
    println!("--- Day 13: Claw Contraption ---");

    let re =
        Regex::new(r".*X.([0-9]+), Y.([0-9]+)\n.*X.([0-9]+), Y.([0-9]+)\n.*X.([0-9]+), Y.([0-9]+)")
            .unwrap();

    let systems: Vec<[i64; 6]> = re
        .captures_iter(input)
        .map(|cap| cap.extract::<6>().1.map(|n| n.parse().unwrap()))
        .collect();

    let mut coins = 0;
    for [a, c, b, d, x, y] in systems.iter() {
        let det = a * d - b * c;
        // one might want to check if
        // determinant is zero here but it never is

        let a_presses = (x * d - y * b) / det;
        let b_presses = (y * a - x * c) / det;

        if a_presses * a + b_presses * b != *x || a_presses * c + b_presses * d != *y {
            continue;
        }

        coins += a_presses * 3 + b_presses;
    }

    println!("Part 1: {}", coins);

    let mut coins = 0;
    for [a, c, b, d, x, y] in systems.iter() {
        let x = *x + 10000000000000;
        let y = *y + 10000000000000;
        let det = a * d - b * c;
        // one might want to check if
        // determinant is zero here but it never is

        let a_presses = (x * d - y * b) / det;
        let b_presses = (y * a - x * c) / det;

        if a_presses * a + b_presses * b != x || a_presses * c + b_presses * d != y {
            continue;
        }

        coins += a_presses * 3 + b_presses;
    }

    println!("Part 1: {}", coins);
}
