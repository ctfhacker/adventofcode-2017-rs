use std::collections::{HashSet, VecDeque};

fn hash(input: &str) -> String {
    let mut input: Vec<_> = input.bytes()
                             .map(|b| b as usize)
                             .collect();

    input.extend(vec!(17, 31, 73, 47, 23));

    let twisted = twist((0..256).collect(), input, 64);
    let result = twisted.chunks(16)   // Chunk the vec of 256 elements into vec of 16 16-sized elements
                        .map(|c| c.iter().fold(0, |acc, &x| acc ^ x)) // xor all elements of each chunk together
                        .map(|n| format!("{:02x}", n).to_string()) // 0 left pad hex convert each digit
                        .collect::<Vec<_>>() // Convert to a Vector
                        .join(""); // Join with no spaces
    result
}

fn twist(mut numbers: Vec<usize>, lengths: Vec<usize>, rounds: usize) -> Vec<usize>{
    let mut index = 0;
    let mut skip = 0;
    let numbers_length = numbers.len();
    for _ in 0..rounds {
        for curr_length in lengths.clone() {
            for x in 0..curr_length/2 {
                numbers.swap((index + x) % numbers_length, (index + curr_length - 1 - x) % numbers_length);
            } index = (index + skip + curr_length) % numbers_length;
            skip += 1;
        }
    }
    numbers
}

fn main() {
    let key = String::from("uugsqrei");
    let mut total = 0;
    let mut valid_coords = HashSet::new();

    for line in 0..128 {
        let mut curr_key = key.clone();
        curr_key.push_str(&format!("-{}", line));
        let curr_hash = hash(&curr_key);
        let hash_bits = curr_hash.chars()
                                 .map(|b| format!("{:#06b}", b.to_digit(16).unwrap()))
                                 .map(|b| b.replace("0b", ""))
                                 // .map(|b| b.chars().filter(|ch| *ch == '1').count())
                                 .collect::<Vec<_>>()
                                 .join("");


        let coords: Vec<_> = hash_bits.chars()
                              .enumerate()
                              .filter(|&(i, ch)| ch == '1')
                              .map(|(i, ch)| valid_coords.insert((i, line)))
                              .collect();
    }

    println!("Task 1 results: {}", valid_coords.len());

    let mut regions = 0;
    let mut curr_coord = (0, 0);
    while valid_coords.len() > 0 {
        {
            curr_coord = *valid_coords.iter().nth(0).expect("Curr coord");
        }

        regions += 1;
        println!("curr: {:?}", curr_coord);

        let mut curr_queue = VecDeque::new();
        curr_queue.push_back(curr_coord);

        while let Some(curr_coord) = curr_queue.pop_front() {
            println!("Len: {}", curr_queue.len());
            if curr_coord.0 > 0 { 
                if valid_coords.remove(&(curr_coord.0 - 1, curr_coord.1)) {
                    curr_queue.push_back((curr_coord.0 - 1, curr_coord.1));
                }
            }

            if curr_coord.1 > 0 { 
                if valid_coords.remove(&(curr_coord.0, curr_coord.1 - 1)) {
                    curr_queue.push_back((curr_coord.0, curr_coord.1 - 1));
                }
            }

            if valid_coords.remove(&(curr_coord.0 + 1, curr_coord.1)) {
                curr_queue.push_back((curr_coord.0 + 1, curr_coord.1));
            }

            if valid_coords.remove(&(curr_coord.0, curr_coord.1 + 1)) {
                curr_queue.push_back((curr_coord.0, curr_coord.1 + 1));
            }

            valid_coords.remove(&curr_coord);
        }
    }
    println!("Task 2 result: {}", regions);

    println!("Task one results: {}", total);
}
