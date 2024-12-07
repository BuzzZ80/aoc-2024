pub fn run(input: &String) {
    let equations_iter = input.lines().map(|line| {
        line.split(|c: char| !c.is_numeric())
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<i64>>()
    });

    let timer1 = std::time::Instant::now();
    let sum: i64 = equations_iter
        .clone()
        .filter_map(|eq| if eq_is_valid(&eq) { Some(eq[0]) } else { None })
        .sum();
    println!("Star 1: {} ({} us)", sum, timer1.elapsed().as_micros());

    let timer2 = std::time::Instant::now();
    let sum: i64 = equations_iter
        .clone()
        .filter_map(|eq| if eq_is_valid_2(&eq) { Some(eq[0]) } else { None })
        .sum();
    println!("Star 2: {} ({} ms)", sum, timer2.elapsed().as_millis());
}

fn eq_is_valid(eq: &Vec<i64>) -> bool {
    let mut eq = eq.iter();
    let ans = *eq.next().expect("Wrong input?");
    let first = *eq.next().expect("Wrong input?");
    let len = eq.clone().count() as u32;
    let mut found = false;
    for operations in 0..2_i64.pow(len) {
        let result = eq.clone().enumerate().fold(first, |accumulator, (i, n)| {
            if ((operations >> i) & 1) == 1 {
                accumulator + n
            } else {
                accumulator * n
            }
        });
        if result == ans {
            found = true;
            break;
        }
    }
    found
}

fn eq_is_valid_2(eq: &Vec<i64>) -> bool {
    let mut eq = eq.iter();
    let ans = *eq.next().expect("Wrong input?");
    let first = *eq.next().expect("Wrong input?");
    let len = eq.clone().count() as u32;
    let mut found = false;
    for operations in 0..3_i64.pow(len) {
        let result = eq.clone().enumerate().fold(first, |accumulator, (i, n)| {
            let op = (operations / 3_i64.pow(i as u32)) % 3;
            match op {
                0 => accumulator + n,
                1 => accumulator * n,
                2 => append(accumulator, *n),
                _ => panic!()
            }
        });
        if result == ans {
            found = true;
            break;
        }
    }
    found
}

fn append(a: i64, b: i64) -> i64 {
    let digits = (b.ilog10() + 1) as u32;
    a * 10i64.pow(digits) + b
}