// FORGIVE MY SPAGHETTI I WAS TRYING TO WRITE IT QUICKLY D:

pub fn run(input: &String) {
    println!("--- Day 2: Red-Nosed Reports ---");

    // Part 1
    // get reports as an iter of Vec<i32>
    let reports = input.lines().map(|l| {
        l.split(|c: char| c.is_whitespace())
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    // For each report, return true if safe and false if not safe
    let safe = reports.clone().map(|r| {
        // unsafe if first two elements are the same
        if r[0] == r[1] {
            return false;
        }
        // otherwise, use them to determine increasing or decreasing
        let increasing = r[0] < r[1];

        // check if all entries are also increasing/decreasing, and if they're within the right range
        for level in 0..r.len() - 1 {
            if (r[level] == r[level + 1])
                || (increasing ^ (r[level] < r[level + 1]))
                || (r[level].abs_diff(r[level + 1]) > 3)
            {
                return false;
            }
        }

        true
    });

    let safe2 = reports.clone().map(|r| {
        // Create iter of each report with different elements removed
        // (including the initial list which is probably not neccessary)
        let versions = (0..r.len())
            .map(|i| {
                let mut removed = r.clone();
                removed.remove(i);
                removed
            })
            .chain(Some(r.clone()));

        // verify each one until one returns true
        // if none return true, return false.
        versions
            .map(|r: Vec<i32>| {
                if r[0] == r[1] {
                    return false;
                }
                let increasing = r[0] < r[1];

                for level in 0..r.len() - 1 {
                    if (r[level] == r[level + 1])
                        || (increasing ^ (r[level] < r[level + 1]))
                        || (r[level].abs_diff(r[level + 1]) > 3)
                    {
                        return false;
                    }
                }

                true
            })
            .any(|b| b)
    });

    // count true/false i.e. safe/unsafe reports
    let mut count = 0;
    safe.for_each(|b| count += b as i32);

    let mut count2 = 0;
    safe2.for_each(|b| count2 += b as i32);

    println!("Star 1: {count}");
    println!("Star 2: {count2}");
}
