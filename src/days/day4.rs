pub fn run(input: &String) {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<u8>>())
        .collect();

    // list of s's in the text
    let mut s_list: Vec<(usize, usize)> = Vec::new();

    // find every s in the grid
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == b'S' {
                s_list.push((row, col));
            }
        }
    }

    let mut accumulator = 0;
    for s in s_list {
        for branch in (0..=8).filter(|n| *n != 4) {
            let first: i32 = branch % 3 - 1; // ranges from -1 to 1
            let second: i32 = branch / 3 - 1; // ranges from -1 to 1

            accumulator += "AMX"
                .bytes()
                .enumerate()
                .map(|(i, v)| {
                    let i = i as i32 + 1;
                    let coords = (
                        (s.0 as i32 + i * first) as usize,
                        (s.1 as i32 + i * second) as usize,
                    );
                    check_char(coords, v, &grid)
                })
                .all(|b| b == true) as i32;
        }
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

fn check_char(coord: (usize, usize), val: u8, grid: &Vec<Vec<u8>>) -> bool {
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
