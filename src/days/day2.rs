// FORGIVE MY SPAGHETTI I WAS TRYING TO WRITE IT QUICKLY D:

pub fn run(input: &String) {
    println!("--- Day 2: Red-Nosed Reports ---");

    // Part 1
    // get reports as an iter of Vec<i32>
    let reports = input.lines().map(|l| {
        l.split(|c: char| c.is_whitespace())
            .map(|n| n.parse::<i32>().expect("Wrong input?"))
            .collect::<Vec<i32>>()
    });

    // Sum up safe reports
    let safe_reports_1: i32 = reports
        .clone()
        .map(|report| find_unsafe_level(&report).is_none() as i32)
        .sum();

    let safe_reports_2: i32 = reports
        .clone()
        .map(|report| {
            if let Some(bad_level) = find_unsafe_level(&report) {
                (find_unsafe_level(&remove(&report, bad_level)).is_none()
                    || find_unsafe_level(&remove(&report, bad_level + 1)).is_none()
                    || (bad_level > 0
                        && find_unsafe_level(&remove(&report, bad_level - 1)).is_none()))
                    as i32
            } else {
                1
            }
        })
        .sum();

    println!("Star 1: {safe_reports_1}");
    println!("Star 2: {safe_reports_2}");
}

/// Returns Some containing i32 if there's an unsafe level.
/// If there are no unsafe levels, return None
/// Assumes report has a length of at least two
fn find_unsafe_level(report: &Vec<i32>) -> Option<usize> {
    // unsafe if first two elements are the same
    if report[0] == report[1] {
        return Some(0);
    }

    // otherwise, the first two elements determine
    // if the report is increasing or decreasing
    let increasing = report[0] < report[1];

    // check if all entries are also increasing/decreasing, and if they're within the right range
    for level in 0..report.len() - 1 {
        if (report[level] == report[level + 1]) // equal levels are unsafe
            || (increasing ^ (report[level] < report[level + 1]))   // must always increase or decrease
            || (report[level].abs_diff(report[level + 1]) > 3)
        // difference must not be too large
        {
            return Some(level);
        }
    }

    // No unsafe level to report
    None
}

fn remove<T: Clone>(input: &Vec<T>, index: usize) -> Vec<T> {
    let mut output = input.clone();
    output.remove(index);
    output
}
