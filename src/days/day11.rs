use std::collections::HashMap;

pub fn run(input: &String) {
    println!("--- Day 11: Plutonian Pebbles ---");

    let mut stones = HashMap::<u64, u64>::new();
    for i in input.split_ascii_whitespace() {
        let n = i.parse().expect("Wrong input?");
        add_to(&mut stones, n, 1);
    }

    for _ in 0..25 {
        sim_step(&mut stones);
    }

    println!("Star 1: {}", stones.iter().map(|(_, v)| *v).sum::<u64>());

    for _ in 0..50 {
        sim_step(&mut stones);
    }

    println!("Star 2: {}", stones.iter().map(|(_, v)| *v).sum::<u64>());
}

fn sim_step(stones: &mut HashMap<u64, u64>) {
    let mut new_stones = HashMap::new();
    for (num, count) in stones.iter() {
        // first rule
        if *num == 0 {
            add_to(&mut new_stones, 1, *count);
        }
        // second rule
        else if num.ilog10() & 1 == 1 {
            let digits = num.ilog10() + 1;
            let factor = 10_u64.pow(digits / 2);
            let left = *num / factor;
            let right = *num % factor;

            add_to(&mut new_stones, left, *count);
            add_to(&mut new_stones, right, *count);
        } else {
            add_to(&mut new_stones, *num * 2024, *count);
        }
    }

    *stones = new_stones;
}

fn add_to(hm: &mut HashMap<u64, u64>, k: u64, v: u64) {
    if hm.contains_key(&k) {
        *hm.get_mut(&k).unwrap() += v;
    } else {
        hm.insert(k, v);
    }
}