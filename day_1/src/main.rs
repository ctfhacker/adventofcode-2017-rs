use std::fs::File;
use std::io::Read;

fn task_1() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test_1").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");
    let result: u32 = contents.chars()                                    // Read the file contents by characters
                              .enumerate()                                // Get an iterator of (index, char)
                              .filter(|&(i, c)| c == contents.chars()
                                                              .nth((i + 1) % contents.chars().count())
                                                              .unwrap())   // Compare the current character to the next character (Circular with mod)
                              .map(|(i, c)| c.to_digit(10).unwrap())      // Convert all found characters to digits
                              .sum();                                     // Sum all found digits

    println!("Result - Task 1: {:?}", result);
}

fn task_2() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test_2").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");
    let input_len = contents.trim().chars().count();
    let result: u32 = contents.trim().chars()                                    // Read the file contents by characters
                                     .enumerate()                                // Get an iterator of (index, char)
                                     .filter(|&(i, c)| c == contents.chars()
                                                                    .nth((i + (input_len / 2)) % input_len)
                                                                    .unwrap())   // Compare the current character to the next character halfway around the input
                                    .map(|(i, c)| c.to_digit(10).unwrap())       // Convert all found characters to digits
                                    .sum();                                      // Sum all found digits

    println!("Result - Task 2: {:?}", result);
}

fn main() {
    task_1();
    task_2();
}
