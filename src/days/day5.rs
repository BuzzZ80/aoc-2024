pub fn run(input: &String) {
    let mut lines = input.lines();

    // Operating on linues until the linebreak,
    // create tuples of page ordering rules
    // and place them in the rules vec
    let mut rules: Vec<(i16, i16)> = Vec::new();
    loop {
        // get next line
        let next_line = lines.next().expect("Wrong input?");

        // break on empty line seperator
        if next_line.len() == 0 {
            break;
        }

        // get index of seperator character, so we can seperate
        // out the numbers
        let divider = next_line.find('|').expect("Wrong input?");

        // parse the numbers and push them to the list
        rules.push((
            next_line[0..divider].parse::<i16>().expect("Wrong input?"),
            next_line[divider + 1..]
                .parse::<i16>()
                .expect("Wrong input?"),
        ));
    }

    // On the remaining lines, seperate the comma seperated lists
    // into one Vec<Vec<i16>>
    let updates = lines
        .map(|line| {
            line.split(',')
                .map(str::parse::<i16>)
                .map(Result::unwrap)
                .collect()
        })
        .collect::<Vec<Vec<i16>>>();

    // sum up all the valid updates
    let sum: i32 = updates
        .iter()
        .filter(|u| {
            // Iterate through all groups of two elements
            // from the update
            for pg1 in 0..u.len() - 1 {
                for pg2 in pg1..u.len() {
                    // if there's a violation, return false
                    if rules.contains(&(u[pg2], u[pg1])) {
                        return false;
                    }
                }
            }
            // no violations found, return true
            true
        })
        .map(|u| u[u.len() / 2] as i32)
        .sum(); // sum up middle entries

    println!("Star 1: {sum}");

    // part 2
    // clone list for modification
    let fixed_updates = updates.clone();
    let mut sum = 0;
    for mut u in fixed_updates {
        // keep track of if the update was already valid
        let mut looped = false;
        // Repeatedly loop through and swap if out of order
        loop {
            // iterate through all groups of two
            for pg1 in 0..u.len() - 1 {
                for pg2 in pg1..u.len() {
                    // if out of order, swap and restart
                    if rules.contains(&(u[pg2], u[pg1])) {
                        let tmp = u[pg1];
                        u[pg1] = u[pg2];
                        u[pg2] = tmp; // swappy :3
                        looped = true;
                        continue;
                    }
                }
            }
            break;
        }
        // if it was invalid before, then count its middle entry
        if looped {
            sum += u[u.len() / 2];
        }
    }

    println!("Star 2: {sum}");
}
