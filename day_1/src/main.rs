use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");
    let result: u32 = contents.chars()                                                  // Read the file contents by characters
                              .enumerate()                                              // Get an iterator of (index, char)
                              .filter(|&(i, c)| {
                                  if i == (contents.chars().count()-1) {                
                                      println!("{}", i);
                                      return c == contents.chars().nth(0).unwrap();     // For the last character, compare it to the first (circular)
                                  } else {
                                      return c == contents.chars().nth(i+1).unwrap();   // Compare the current character to the next character
                                  }
                              })
                              .map(|(i, c)| c.to_digit(10).unwrap())                    // Convert all found characters to digits
                              .sum();                                                   // Sum all found digits

    println!("Result: {:?}", result);
}
