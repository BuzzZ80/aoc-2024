pub fn run(input: &String) {
    println!("--- Day 12: Garden Groups ---");

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut original: Vec<Vec<u32>> = Vec::new();
    let mut map: Vec<Vec<u32>> = Vec::new();

    let mut in_iter = input.bytes().filter(|c| !c.is_ascii_whitespace());
    for _y in 0..height {
        let mut tmp1 = Vec::with_capacity(width);
        let mut tmp2 = Vec::with_capacity(width);
        for _x in 0..width {
            tmp1.push(in_iter.next().unwrap() as u32);
            tmp2.push(0);
        }
        original.push(tmp1);
        map.push(tmp2);
    }

    let mut next_id = 1;
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 0 {
                map[y][x] = next_id;
                let (area, perimeter) = fill(&mut map, &original, (y, x), (width, height), next_id);
                println!("{area} {perimeter} {}", area * perimeter);
                total += area * perimeter;
                next_id += 1;
            }
        }
    }

    println!("Star 1: {total}");
}

fn fill(out: &mut Vec<Vec<u32>>, input: &Vec<Vec<u32>>, start: (usize, usize), size: (usize, usize), value: u32) -> (u32, u32) {
    let mut area = 1;
    let mut perimeter = 0;

    let chr = input[start.0][start.1];
    if grid_check(&input, chr, (start.0 - 1, start.1)) && grid_check(&out, 0, (start.0 - 1, start.1)) {
        out[start.0 - 1][start.1] = value;
        let (add_area, add_perimeter) = fill(out, input, (start.0 - 1, start.1), size, value);
        area += add_area;
        perimeter += add_perimeter;
    } else {
        perimeter += !grid_check(&input, chr, (start.0 - 1, start.1)) as u32;
    }
    if grid_check(&input, chr, (start.0 + 1, start.1)) && grid_check(&out, 0, (start.0 + 1, start.1)) {
        out[start.0 + 1][start.1] = value;
        let (add_area, add_perimeter) = fill(out, input, (start.0 + 1, start.1), size, value);
        area += add_area;
        perimeter += add_perimeter;
    } else {
        perimeter += !grid_check(&input, chr, (start.0 + 1, start.1)) as u32;
    }
    if grid_check(&input, chr, (start.0, start.1 - 1)) && grid_check(&out, 0, (start.0, start.1 - 1)) {
        out[start.0][start.1 - 1] = value;
        let (add_area, add_perimeter) = fill(out, input, (start.0, start.1 - 1), size, value);
        area += add_area;
        perimeter += add_perimeter;
    } else {
        perimeter += !grid_check(&input, chr, (start.0, start.1 - 1)) as u32;
    }
    if grid_check(&input, chr, (start.0, start.1 + 1)) && grid_check(&out, 0, (start.0, start.1 + 1)) {
        out[start.0][start.1 + 1] = value;
        let (add_area, add_perimeter) = fill(out, input, (start.0, start.1 + 1), size, value);
        area += add_area;
        perimeter += add_perimeter;
    } else {
        perimeter += !grid_check(&input, chr, (start.0, start.1 + 1)) as u32;
    }

    (area, perimeter)
}

fn grid_check(grid: &Vec<Vec<u32>>, val: u32, pos: (usize, usize)) -> bool {
    if let Some(Some(content)) = grid.get(pos.0).map(|o| o.get(pos.1)) {
        *content == val
    } else {
        false
    }
}