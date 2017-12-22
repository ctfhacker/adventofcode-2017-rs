extern crate permutohedron;

use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use permutohedron::Heap;
use std::iter::FromIterator;

fn task_1() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");

    let mut result = 0;
    for line in contents.trim().lines() {
        let mut hashset = HashSet::new();
        if line.split_whitespace().all(|s| hashset.insert(s)) {
            result += 1;
        }
    }

    println!("Result task 1: {}", result);
}

fn task_2() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");

    let mut result = 0;
    let num_lines = contents.trim().lines().count();
    for (i, line) in contents.trim().lines().enumerate() {
        let mut valid = true;
        let mut hashset = HashSet::new();
        for word in line.split_whitespace() {
            let mut word_vec = word.chars().collect::<Vec<char>>();
            let heap = Heap::new(&mut word_vec);

            // permutohedron produces permutations that are not unique
            // ee -> vec!(['e', 'e'], ['e', 'e'])
            // Isolate all the permutations by throwing results in a HashSet
            let mut permutations = HashSet::new();
            for word in heap {
                permutations.insert(word);
            }

            for anagram in permutations {
                if !hashset.insert(anagram) {
                    valid = false;
                }
            }
        }

        if valid {
            result += 1; 
        }
    }

    println!("Result task 2: {}", result);
}

fn main() {
    task_1();
    task_2();
}
