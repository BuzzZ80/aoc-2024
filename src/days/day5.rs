struct Page(i16);

pub fn run(input: &String) {
    let mut lines = input.lines();

    let mut rules: Vec<(i16, i16)> = Vec::new();
    loop {
        let next_line = lines.next().expect("Wrong input?");

        if next_line.len() == 0 {
            break;
        }

        let divider = next_line.find('|').unwrap();

        rules.push((
            next_line[0..divider].parse::<i16>().unwrap(),
            next_line[divider + 1..].parse::<i16>().unwrap(),
        ));
    }

    let updates = lines
        .map(|line| {
            line.split(',')
                .map(str::parse::<i16>)
                .map(Result::unwrap)
                .collect()
        })
        .collect::<Vec<Vec<i16>>>();

    let sum: i32 = updates.iter().filter(|u| {
        for pg1 in 0..u.len() - 1 {
            for pg2 in pg1..u.len() {
                if rules.contains(&(u[pg2], u[pg1])) {
                    return false;
                }
            }
        }
        true
    }).map(|u| u[u.len()/2] as i32).sum();

    println!("Star 1: {sum}");

    let fixed_updates = updates.clone();
    let mut sum = 0;
    for mut u in fixed_updates {
        let mut looped = false;
        loop {
            for pg1 in 0..u.len() - 1 {
                for pg2 in pg1..u.len() {
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
        if looped {
            sum += u[u.len() / 2];
        }
    }

    println!("Star 2: {sum}");
}
