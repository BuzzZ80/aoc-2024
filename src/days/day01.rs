use std::collections::HashMap;

pub fn run(input: &String) {
    println!("--- Day 1: Historian Hysteria ---");

    // part 1
    // left list and right list, respectively
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    input
        .split(|c: char| c.is_whitespace()) // seperate by whitespace
        .filter(|num_as_str| !num_as_str.is_empty()) // get rid of empty strings
        .map(|num_as_str: &str| num_as_str.parse().expect("Wrong input?")) // parse to integers
        .enumerate() // get index to determine if it's left or right
        .for_each(|(index, n)| {
            // place into appropriate list
            if index % 2 == 0 {
                list1.push(n);
            } else {
                list2.push(n);
            }
        });

    // sort the lists so they can be compared
    list1.sort();
    list2.sort();

    // sum the differences
    let total1: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(n1, n2)| (n1 - n2).abs())
        .sum();

    println!("Star 1 : {total1}");

    // pt 2
    // Count how many times each number appears using a hash map <3
    let mut list2_appearances: HashMap<i32, i32> = HashMap::new();
    for n in list2 {
        if list2_appearances.contains_key(&n) {
            // increment if entry already exists
            *list2_appearances.get_mut(&n).unwrap() += 1;
        } else {
            // create new entry if there was none
            list2_appearances.insert(n, 1);
        }
    }

    // accumulate number times number of appearances for similarity score
    let mut total2 = 0;
    for n in list1 {
        if let Some(appearances) = list2_appearances.get(&n) {
            total2 += n * appearances;
        }
    }

    println!("Star 2 : {total2}")
}
