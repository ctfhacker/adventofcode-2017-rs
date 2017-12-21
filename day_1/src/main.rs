use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");
    let result: u32 = contents.chars()
                              .enumerate()
                              .filter(|&(i, c)| {
                                  if i == (contents.chars().count()-1) { 
                                      println!("{}", i);
                                      return c == contents.chars().nth(0).unwrap();
                                  } else {
                                      return c == contents.chars().nth(i+1).unwrap();
                                  }
                              })
                              .map(|(i, c)| c.to_digit(10).unwrap())
                              .sum();

    println!("Result: {:?}", result);
}
