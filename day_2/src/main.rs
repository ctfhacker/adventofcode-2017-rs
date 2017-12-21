use std::fs::File;
use std::io::Read;

fn main() {
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

    println!("Result: {:?}", result);
}
