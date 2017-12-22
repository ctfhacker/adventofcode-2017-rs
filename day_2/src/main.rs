use std::fs::File;
use std::io::Read;

fn task_1() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");

    let result: u32 = contents.trim().split('\n')                                                   // Split lines
                                      .map(|l|l.split_whitespace()                                  // Split each line by whitespace
                                               .filter_map(|c| c.parse().ok())                      // Convert each character from string to number
                                               .collect::<Vec<u32>>())                              // Collect those digits back into a vector
                                      .map(|l| l.iter().max().unwrap() - l.iter().min().unwrap())   // Calculate each lines max - min
                                      .sum();                                                       // Sum all differences together

    println!("Result task 1 -- {:?}", result);
}

fn task_2() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");

    let result: u32 = contents.trim().split('\n')                                                         // Split lines
                                     .map(|l|l.split_whitespace()                                  // Split each line by whitespace
                                              .filter_map(|c| c.parse().ok())                      // Convert each character from string to number
                                              .collect::<Vec<u32>>())                              // Collect those digits back into a vector
                                     .map(|l| l.iter()                                             // Iterate over the current line
                                               .map(|&x| l.iter()                                  // For each digit in the same line
                                                          .filter(|&y| x != *y && x % *y == 0)     // Check for the current digit divisible by other digits (and not itself)
                                                          .map(|n| x / n)                          // Retrieve that dividend
                                                          .sum::<u32>())                           // Sum the results for that number
                                               .sum::<u32>())                                      // Sum the results for the line
                                     .sum();                                                       // Sum the results for the grid

    println!("Result task 2 -- {:?}", result);
}

fn main() {
    task_2()
}
