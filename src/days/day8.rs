use nalgebra::Vector2;
use std::collections::{HashMap, HashSet};

pub fn run(input: &String) {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;

    let mut antennae: HashMap<u8, Vec<Vector2<i32>>> = HashMap::new();
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, ch)| *ch != b'.')
                .map(move |(x, ch)| (Vector2::new(x as i32, y as i32), ch))
        })
        .for_each(|(p, ch)| {
            if antennae.contains_key(&ch) {
                antennae.get_mut(&ch).unwrap().push(p);
            } else {
                antennae.insert(ch, vec![p]);
            }
        });

    let mut points: HashSet<Vector2<i32>> = HashSet::new();
    for (_, freq_group) in antennae {
        for i in 0..freq_group.len() - 1 {
            for j in i + 1..freq_group.len() {
                for p in get_antinodes([freq_group[i], freq_group[j]]) {
                    if (0..width).contains(&p.x) && (0..height).contains(&p.y) {
                        points.insert(p);
                    }
                }
            }
        }
    }

    println!("Star 1: {}", points.len())
}

fn get_antinodes(p: [Vector2<i32>; 2]) -> [Vector2<i32>; 2] {
    [2 * p[1] - p[0], 2 * p[0] - p[1]]
}
