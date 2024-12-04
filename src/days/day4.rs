pub fn run(input: &String) {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<u8>>())
        .collect();

    // list of s's in the text
    let mut s_list: Vec<(i32, i32)> = Vec::new();

    // find every s in the grid
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == b'S' {
                s_list.push((row as i32, col as i32));
            }
        }
    }

    let mut accumulator = 0;
    for s in s_list {
        // left check
        accumulator += "XMA"
            .bytes()
            .enumerate()
            .map(|(a, b)| (a as i32, b))
            .map(|(i, v)| check_char((s.0, s.1 - 3 + i), v, &grid))
            .all(|b| b == true) as i32;
        // up_left check
        accumulator += "XMA"
            .bytes()
            .enumerate()
            .map(|(a, b)| (a as i32, b))
            .map(|(i, v)| check_char((s.0 + 3 - i, s.1 - 3 + i), v, &grid))
            .all(|b| b == true) as i32;
        // down_left check
        accumulator += "XMA"
            .bytes()
            .enumerate()
            .map(|(a, b)| (a as i32, b))
            .map(|(i, v)| check_char((s.0 - 3 + i, s.1 - 3 + i), v, &grid))
            .all(|b| b == true) as i32;
        // up check
        accumulator += "XMA"
            .bytes()
            .enumerate()
            .map(|(a, b)| (a as i32, b))
            .map(|(i, v)| check_char((s.0 + 3 - i, s.1), v, &grid))
            .all(|b| b == true) as i32;
        // down check
        accumulator += "XMA"
            .bytes()
            .enumerate()
            .map(|(a, b)| (a as i32, b))
            .map(|(i, v)| check_char((s.0 - 3 + i, s.1), v, &grid))
            .all(|b| b == true) as i32;
        // right check
        accumulator += "XMA"
            .bytes()
            .enumerate()
            .map(|(a, b)| (a as i32, b))
            .map(|(i, v)| check_char((s.0, s.1 + 3 - i), v, &grid))
            .all(|b| b == true) as i32;
        // up right check
        accumulator += "XMA"
            .bytes()
            .enumerate()
            .map(|(a, b)| (a as i32, b))
            .map(|(i, v)| check_char((s.0 + 3 - i, s.1 + 3 - i), v, &grid))
            .all(|b| b == true) as i32;
        // down right check
        accumulator += "XMA"
            .bytes()
            .enumerate()
            .map(|(a, b)| (a as i32, b))
            .map(|(i, v)| check_char((s.0 - 3 + i, s.1 + 3 - i), v, &grid))
            .all(|b| b == true) as i32;
    }

    println!("Star 1: {}", accumulator);

    // find every A in the grid
    let mut a_list: Vec<(i32, i32)> = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == b'A' {
                a_list.push((row as i32, col as i32));
            }
        }
    }

    let mut accumulator = 0;
    for a in a_list {
        if let (Some(a1), Some(a2), Some(b1), Some(b2)) = (
            get_char((a.0 - 1, a.1 - 1), &grid),
            get_char((a.0 + 1, a.1 + 1), &grid),
            get_char((a.0 + 1, a.1 - 1), &grid),
            get_char((a.0 - 1, a.1 + 1), &grid),
        ) {
            accumulator += (((a1 == b'M' && a2 == b'S') || (a1 == b'S' && a2 == b'M'))
                && ((b1 == b'M' && b2 == b'S') || (b1 == b'S' && b2 == b'M')))
                as i32;
        }
    }
    println!("Star 2: {}", accumulator);
}

fn check_char(coord: (i32, i32), val: u8, grid: &Vec<Vec<u8>>) -> bool {
    if let Some(Some(read_value)) = grid
        .get(coord.0 as usize)
        .map(|col| col.get(coord.1 as usize))
    {
        *read_value == val
    } else {
        false
    }
}

fn get_char(coord: (i32, i32), grid: &Vec<Vec<u8>>) -> Option<u8> {
    if let Some(Some(read_value)) = grid
        .get(coord.0 as usize)
        .map(|col| col.get(coord.1 as usize))
    {
        Some(*read_value)
    } else {
        None
    }
}
