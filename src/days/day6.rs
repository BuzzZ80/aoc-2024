use std::collections::HashSet;

pub fn run(input: &String) {
    // parse input
    // get dimensions of grid
    let grid_width = input.lines().next().expect("Wrong input?").len() as i32;
    let grid_height = input.lines().collect::<Vec<_>>().len() as i32;

    // locate `^` symbol
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

    // locate all `#` symbols and place them in a hash set
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

    // part 1 solution
    let path = get_full_path(
        ((initial_guard_pos), 0),
        obstructions.clone(),
        grid_width,
        grid_height,
    )
    .expect("input contains loop");
    println!("Star 1: {}", path.len());

    // part 2 solution
    // try placing an obstacle in every place visited by the guard
    let configurations: i32 = path
        .iter()
        .filter(|(obs_x, obs_y)| !(*obs_x == initial_guard_pos.0 && *obs_y == initial_guard_pos.1))
        .map(|(obs_x, obs_y)| {
            let mut new_obstructions = obstructions.clone();
            new_obstructions.insert((*obs_x, *obs_y));
            get_full_path(
                ((initial_guard_pos), 0),
                new_obstructions,
                grid_width,
                grid_height,
            )
            .is_none() as i32
        })
        .sum();
    println!("Star 2: {}", configurations);
}

/// returns none if loop is detected
/// otherwise, returns whole path as a hashset
fn get_full_path(
    start_state: ((i32, i32), i32),
    obstructions: HashSet<(i32, i32)>,
    grid_width: i32,
    grid_height: i32,
) -> Option<HashSet<(i32, i32)>> {
    // initialize mutable state (guard state, visited square list, and all previous guard state list)
    let (mut guard_pos, mut guard_dir) = start_state; // guard state
    let mut visited: HashSet<(i32, i32)> = HashSet::new(); // for path length calculation
    let mut guard_prev_states: HashSet<((i32, i32), i32)> = HashSet::new(); // for loop detection

    // keep looping until loop is detected or guard leaves area
    loop {
        // guard left check, then loop check
        if guard_pos.0 < 0
            || guard_pos.1 < 0
            || guard_pos.0 >= grid_width as i32
            || guard_pos.1 >= grid_height as i32
        {
            break Some(visited);
        } else if guard_prev_states.contains(&(guard_pos, guard_dir)) {
            break None;
        }

        // update squares visited list and prev state list
        visited.insert(guard_pos);
        guard_prev_states.insert((guard_pos, guard_dir));

        // Check if there's an obstruction in front of the guard
        // 0 -> up, 1 -> right, 2 -> down, 3 -> left
        let obstructed = match guard_dir {
            0 => obstructions.contains(&(guard_pos.0, guard_pos.1 - 1)),
            1 => obstructions.contains(&(guard_pos.0 + 1, guard_pos.1)),
            2 => obstructions.contains(&(guard_pos.0, guard_pos.1 + 1)),
            3 => obstructions.contains(&(guard_pos.0 - 1, guard_pos.1)),
            _ => panic!("you goofed it lol"),
        };

        // If there was an obstruction, turn, then loop
        // if there was no obstruction, move forward :)
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
}
