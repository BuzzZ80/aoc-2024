use std::collections::VecDeque;

pub fn run(input: &String) {
    println!("--- Day 9: Disk Fragmenter ---");

    let disk_map: Vec<u8> = input.bytes().collect();

    // part 1
    let mut disk: Vec<u64> = Vec::new();
    // Vec<(file id, length)>
    let mut file_ranges: VecDeque<(u64, u8)> = disk_map
        .iter()
        .step_by(2)
        .enumerate()
        .map(|(n, l)| (n as u64, *l - b'0'))
        .collect();
    // lengths
    let mut empty_ranges = disk_map.iter().skip(1).step_by(2).map(|l| *l - b'0');

    loop {
        if let Some((id, len)) = file_ranges.pop_front() {
            for _ in 0..len {
                disk.push(id);
            }
        }

        if file_ranges.len() == 0 {
            break;
        }

        if let Some(len) = empty_ranges.next() {
            let mut current_file = file_ranges.len() - 1;
            for _ in 0..len {
                disk.push(file_ranges[current_file].0);
                file_ranges[current_file].1 -= 1;

                if file_ranges[current_file].1 == 0 {
                    if current_file == 0 {
                        break;
                    }
                    current_file -= 1;
                    file_ranges.pop_back();
                }
            }
        }
    }

    let result: u64 = disk
        .iter()
        .enumerate()
        .map(|(pos, id)| pos as u64 * id)
        .sum();

    println!("Star 1: {result}");

    // part 2
    // Vec<(file id, disk_index, length)>
    let mut file_ranges: VecDeque<(u64, u32, u8)> = VecDeque::new();
    let mut fileid: u64 = 0;
    let mut diskix: u32 = 0;
    for (i, len) in disk_map
        .iter()
        .filter(|c| c.is_ascii_alphanumeric())
        .enumerate()
    {
        if i & 1 == 0 {
            file_ranges.push_back((fileid, diskix, *len - b'0'));
            fileid += 1;
        }
        diskix += (*len - b'0') as u32;
    }

    let mut i = file_ranges.len() - 1;
    while i > 0 {
        let test_len = file_ranges[i].2;
        for j in 0..i {
            let f1end = file_ranges[j].1 + file_ranges[j].2 as u32;
            let f2start = file_ranges[j + 1].1;
            if f2start - f1end >= test_len as u32 {
                let mut entry = file_ranges.remove(i).unwrap();
                entry.1 = f1end;
                file_ranges.insert(j + 1, entry);
                i += 1;
                break;
            }
        }
        i -= 1;
    }

    let result: u64 = file_ranges
        .iter()
        .map(|(id, ix, len)| id * (*ix..(ix + *len as u32)).sum::<u32>() as u64)
        .sum();

    println!("Star 2: {result}");
}
