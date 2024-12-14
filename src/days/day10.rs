use std::collections::HashSet;

pub fn run(input: &String) {
    println!("--- Day 10: Hoof It ---");

    let width = input.lines().next().expect("Wrong Input?").len();
    let height = input.lines().count();

    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();

    let all_coords = (0..height).flat_map(|col| (0..width).map(move |row| (col, row)));

    let total: usize = all_coords
        .clone()
        .filter(|(col, row)| map[*col][*row] == 0)
        .map(|coord| trail_nines(0, coord, &map).len())
        .sum();

    println!("Star 1: {}", total);

    let total: u32 = all_coords
        .clone()
        .filter(|(col, row)| map[*col][*row] == 0)
        .map(|coord| trail_rating(0, coord, &map) as u32)
        .sum();

    println!("Star 1: {}", total);
}

fn trail_nines(height: u8, pos: (usize, usize), map: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    if height == 9 {
        return HashSet::from_iter(std::iter::once(pos));
    }

    (-1..=1)
        .flat_map(|col| (-1..=1).map(move |row| (col, row)))
        .filter(|(y, x)| (*x == 0) ^ (*y == 0))
        .map(|(y, x)| (pos.0 as i32 + y, pos.1 as i32 + x))
        .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < map.len() as i32 && *x < map[0].len() as i32)
        .map(|(y, x)| (y as usize, x as usize))
        .filter(|(y, x)| map[*y][*x] == height + 1)
        .flat_map(|coord| trail_nines(height + 1, coord, &map))
        .collect()
}

fn trail_rating(height: u8, pos: (usize, usize), map: &Vec<Vec<u8>>) -> u8 {
    if height == 9 {
        return 1;
    }

    (-1..=1)
        .flat_map(|col| (-1..=1).map(move |row| (col, row)))
        .filter(|(y, x)| (*x == 0) ^ (*y == 0))
        .map(|(y, x)| (pos.0 as i32 + y, pos.1 as i32 + x))
        .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < map.len() as i32 && *x < map[0].len() as i32)
        .map(|(y, x)| (y as usize, x as usize))
        .filter(|(y, x)| map[*y][*x] == height + 1)
        .map(|coord| trail_rating(height + 1, coord, &map))
        .sum()
}
