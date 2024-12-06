use std::collections::HashSet;

pub fn run(input: &String) {
    let grid_width = input.lines().next().expect("Wrong input?").len();
    let grid_height = input.lines().collect::<Vec<_>>().len();

    let mut guard_dir: i32 = 0;
    let initial_guard_pos: (i32, i32) = input
        .lines()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.bytes()
                .enumerate()
                .filter(move |(_, b)| *b == b'^')
                .map(move |(col, _)| (col as i32, row as i32))
        })
        .next()
        .expect("Wrong input?");
    let mut guard_pos = initial_guard_pos;

    let obstructions: HashSet<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.bytes()
                .enumerate()
                .filter(move |(_, b)| *b == b'#')
                .map(move |(col, _)| (col as i32, row as i32))
        })
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    loop {
        if guard_pos.0 < 0
            || guard_pos.1 < 0
            || guard_pos.0 >= grid_width as i32
            || guard_pos.1 >= grid_height as i32
        {
            break;
        }

        visited.insert(guard_pos);

        let obstructed = match guard_dir {
            0 => obstructions.contains(&(guard_pos.0, guard_pos.1 - 1)),
            1 => obstructions.contains(&(guard_pos.0 + 1, guard_pos.1)),
            2 => obstructions.contains(&(guard_pos.0, guard_pos.1 + 1)),
            3 => obstructions.contains(&(guard_pos.0 - 1, guard_pos.1)),
            _ => panic!("you goofed it lol"),
        };

        if obstructed {
            guard_dir = (guard_dir + 1) % 4;
        } else {
            guard_pos = match guard_dir {
                0 => (guard_pos.0, guard_pos.1 - 1),
                1 => (guard_pos.0 + 1, guard_pos.1),
                2 => (guard_pos.0, guard_pos.1 + 1),
                3 => (guard_pos.0 - 1, guard_pos.1),
                _ => panic!("you goofed it lol"),
            }
        }
    }

    println!("Star 1: {}", visited.len());

    // part 2
    let mut count2 = 0;
    for (obs_x, obs_y) in visited {
        guard_dir = 0;
        guard_pos = initial_guard_pos;
        if obs_x == guard_pos.0 && obs_y == guard_pos.1 { continue; }
        let mut guard_prev_states: HashSet<((i32, i32), i32)> = HashSet::new();
        let mut new_obstructions = obstructions.clone();
        new_obstructions.insert((obs_x, obs_y));
        let causes_loop = loop {
            if guard_pos.0 < 0
                || guard_pos.1 < 0
                || guard_pos.0 >= grid_width as i32
                || guard_pos.1 >= grid_height as i32
            {
                break false;
            } else if guard_prev_states.contains(&(guard_pos, guard_dir)){
                break true;
            }
    
            guard_prev_states.insert((guard_pos, guard_dir));
            
    
            let obstructed = match guard_dir {
                0 => new_obstructions.contains(&(guard_pos.0, guard_pos.1 - 1)),
                1 => new_obstructions.contains(&(guard_pos.0 + 1, guard_pos.1)),
                2 => new_obstructions.contains(&(guard_pos.0, guard_pos.1 + 1)),
                3 => new_obstructions.contains(&(guard_pos.0 - 1, guard_pos.1)),
                _ => panic!("you goofed it lol"),
            };
    
            if obstructed {
                guard_dir = (guard_dir + 1) % 4;
            } else {
                guard_pos = match guard_dir {
                    0 => (guard_pos.0, guard_pos.1 - 1),
                    1 => (guard_pos.0 + 1, guard_pos.1),
                    2 => (guard_pos.0, guard_pos.1 + 1),
                    3 => (guard_pos.0 - 1, guard_pos.1),
                    _ => panic!("you goofed it lol"),
                }
            }
        };  

        count2 += causes_loop as i32;
    }
    println!("Star 2: {}", count2);
}
