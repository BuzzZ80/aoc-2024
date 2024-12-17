use std::collections::HashSet;

use nalgebra::Vector2;

/// THIS IS VERY SKETCHY :(
pub fn run(input: &String) {
    println!("--- Day 15: Warehouse Woes ---");

    let mut in_lines = input.lines();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut pos = Vector2::<i32>::new(0, 0);
    let mut row = 0;
    while let Some(line) = in_lines.next() {
        if line.len() == 0 {
            break;
        }
        let line_vec = line
            .bytes()
            .enumerate()
            .map(|(col, c)| {
                if c == b'@' {
                    pos = Vector2::new(row as i32, col as i32);
                }
                c
            })
            .collect();

        grid.push(line_vec);
        row += 1;
    }

    let instructions: Vec<u8> = in_lines.clone().flat_map(|l| l.bytes()).collect();

    // part 1
    for d in instructions.iter() {
        let direction = match d {
            b'^' => Vector2::new(0, -1),
            b'>' => Vector2::new(1, 0),
            b'v' => Vector2::new(0, 1),
            b'<' => Vector2::new(-1, 0),
            _ => panic!("Wrong input?"),
        };

        move_tile(&mut grid, &mut pos, direction);
    }

    let gps_sum = grid
        .iter()
        .enumerate()
        .flat_map(|(row, col_content)| {
            col_content.iter().enumerate().map(
                move |(col, b)| {
                    if *b == b'O' {
                        100 * row + col
                    } else {
                        0
                    }
                },
            )
        })
        .sum::<usize>();

    println!("Star 1: {}", gps_sum);

    // part 2
    let mut in_lines = input.lines();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut pos = Vector2::<i32>::new(0, 0);
    let mut row = 0;
    while let Some(line) = in_lines.next() {
        if line.len() == 0 {
            break;
        }
        let mut line_vec = Vec::new();
        line.bytes().enumerate().for_each(|(col, c)| {
            if c == b'@' {
                pos = Vector2::new(2 * col as i32, row as i32);
            }
            match c {
                b'#' => {
                    line_vec.push(b'#');
                    line_vec.push(b'#');
                }
                b'.' => {
                    line_vec.push(b'.');
                    line_vec.push(b'.');
                }
                b'O' => {
                    line_vec.push(b'[');
                    line_vec.push(b']');
                }
                b'@' => {
                    line_vec.push(b'@');
                    line_vec.push(b'.');
                }
                c => panic!("Wrong input? (found {} in grid)", c as char),
            }
        });

        grid.push(line_vec);
        row += 1;
    }

    for d in instructions.iter() {
        let direction = match d {
            b'^' => Vector2::new(0, -1),
            b'>' => Vector2::new(1, 0),
            b'v' => Vector2::new(0, 1),
            b'<' => Vector2::new(-1, 0),
            _ => panic!("Wrong input?"),
        };

        move_but_more_complicated(&mut grid, &mut pos, direction);
        println!("{}", *d as char);
        //print_grid(&grid);
        //let mut x = String::new();
        //std::io::stdin().read_line(&mut x).unwrap();
    }

    print_grid(&grid);

    let gps_sum = grid
        .iter()
        .enumerate()
        .flat_map(|(row, col_content)| {
            col_content.iter().enumerate().map(
                move |(col, b)| {
                    if *b == b'[' {
                        100 * row + col
                    } else {
                        0
                    }
                },
            )
        })
        .sum::<usize>();

    println!("Star 2: {}", gps_sum);
}

fn move_tile(grid: &mut Vec<Vec<u8>>, init: &mut Vector2<i32>, direction: Vector2<i32>) {
    let mut pos = *init + direction;

    while [b'O', b'[', b']'].contains(&grid[pos.y as usize][pos.x as usize]) {
        pos += direction;
    }

    if grid[pos.y as usize][pos.x as usize] == b'.' {
        while pos != *init {
            let next = pos - direction;
            grid[pos.y as usize][pos.x as usize] = grid[next.y as usize][next.x as usize];
            pos = next;
        }
        grid[init.y as usize][init.x as usize] = b'.';
        *init += direction;
    }
}

fn move_but_more_complicated(
    grid: &mut Vec<Vec<u8>>,
    pos: &mut Vector2<i32>,
    direction: Vector2<i32>,
) {
    if direction.y == 0 {
        move_tile(grid, pos, direction);
        return;
    }

    let mut move_candidates: Vec<Vector2<i32>> = Vec::new();
    move_candidates.push(*pos);

    let mut i = 0;
    while i < move_candidates.len() {
        let p = move_candidates[i];
        let valid = match grid[p.y as usize][p.x as usize] {
            b'[' => {
                let p1 = p + direction;
                let p2 = p + Vector2::new(1, 0);
                move_candidates.push(p1);
                if !move_candidates.contains(&p2) {
                    move_candidates.push(p2);
                }
                grid[p1.y as usize][p1.x as usize] != b'#'
            }
            b']' => {
                let p1 = p + direction;
                let p2 = p + Vector2::new(-1, 0);
                move_candidates.push(p1);
                if !move_candidates.contains(&p2) {
                    move_candidates.push(p2);
                }
                grid[p1.y as usize][p1.x as usize] != b'#'
            }
            b'#' => false,
            b'.' => {
                move_candidates.remove(i);
                i -= 1;
                true
            }
            b'@' => {
                let p1 = p + direction;
                move_candidates.push(p1);
                grid[p1.y as usize][p1.x as usize] != b'#'
            }
            _ => panic!("wrong input"),
        };
        i += 1;
        if !valid {
            move_candidates.clear();
            break;
        }
    }

    if i == move_candidates.len() {
        dedup_real(&mut move_candidates);
        for c1 in move_candidates.iter().rev() {
            println!("{c1:?}");
            let c2 = c1 + direction;
            grid[c2.y as usize][c2.x as usize] = grid[c1.y as usize][c1.x as usize];
            grid[c1.y as usize][c1.x as usize] = b'.';
        }
        *pos += direction;
    }
}

fn print_grid(grid: &Vec<Vec<u8>>) {
    grid.iter()
        .map(|l| String::from_utf8(l.to_owned()).unwrap())
        .for_each(|l| println!("{}", l));
}

fn dedup_real(vec: &mut Vec<Vector2<i32>>) {
    let mut seen = HashSet::<Vector2<i32>>::new();

    let mut i = 0;
    while i < vec.len() {
        if seen.contains(&vec[i]) {
            vec.remove(i);
        } else {
            seen.insert(vec[i]);
            i += 1;
        }
    }
}
