#![feature(nll)]
#![feature(try_trait)]
use std::fs::File;
use std::io::{self, Read};
use std::result::Result;

struct Dance {
    programs: Vec<char>
}

impl Dance {
    fn new(size: u8) -> Dance {
        let mut programs = Vec::new();

        for _i in 0..size {
            programs.push((('a' as u8) + _i) as char);
        }

        Dance {
            programs: programs
        }
    }
    
    fn spin(&mut self, size: usize) {
        for _ in 0..size {
            let last_elem = self.programs.pop().unwrap();
            self.programs.insert(0, last_elem);
        }
    }

    fn exchange(&mut self, a: usize, b: usize) {
        if a >= self.programs.len() || b >= self.programs.len() { 
            panic!("Exchange indexes too large for string"); 
        }

        self.programs.swap(a, b);
    }

    fn partner(&mut self, a: char, b: char) {
        if let Some(a_index) = self.programs.iter().position(|ch| *ch == a) {
            if let Some(b_index) = self.programs.iter().position(|ch| *ch == b) {
                self.programs.swap(a_index, b_index);
            }
        }

    }

    fn execute(&mut self, instrs: &String) {
        for instr in instrs.trim().split(",") {
            match instr.split_at(1) {
                ("s", i) => { 
                    // println!("Spin: {}", i); 
                    self.spin(i.parse::<usize>().ok().unwrap());
                },
                ("x", i) => { 
                    let nums: Vec<_> = i.split(|ch| ch == '/')
                                        .map(|ch| ch.parse::<usize>().ok().unwrap())
                                        .collect();
                    let (a, b) = (nums[0], nums[1]);
                    // println!("Exchange: {}, {}", a, b); 
                    self.exchange(a, b);
                },
                ("p", i) => { 
                    let nums: Vec<_> = i.split(|ch| ch == '/')
                                        .map(|s| s.chars().take(1).next().unwrap())
                                        .collect();
                    let (a, b) = (nums[0], nums[1]);
                    // println!("Partner: {:?}, {:?}", a, b); 
                    self.partner(a, b);
                },
                _ => panic!(format!("Received invalid instr: {}", instr))

            }
        }
    }

    fn programs(&self) -> String {
        self.programs.iter().collect()
    }
}

fn get_input(filename: &str) -> Result<String, io::Error> {
    let mut input_file = File::open("./input")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    Ok(input)
}

fn get_cycles(dance: &mut Dance, instr: &String) -> usize {
    let init = dance.programs();
    for i in 1.. {
        dance.execute(instr);
        if dance.programs() == init {
            println!("Cycles in {}", i);
            return i;
        }
    }
    0xdeadbeef
}

fn main() {
    let input = get_input("./input");
    match input {
        Ok(instr) => { 
            let mut dance = Dance::new(16);
            // dance.execute(&instr);
            println!("First Result: {}", dance.programs());

            for i in 0..(1_000_000_000 % get_cycles(&mut dance, &instr)) {
                dance.execute(&instr);
            }
            println!("{}", dance.programs());
        },
        Err(e) => { println!("Failed to open file: {}", e); }
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dance = Dance::new(5);
        assert_eq!(dance.programs, vec!('a', 'b', 'c', 'd', 'e'));
    }

    #[test]
    fn test_spin() {
        let mut dance = Dance::new(5);
        dance.spin(1);
        assert_eq!(dance.programs, vec!('e', 'a', 'b', 'c', 'd'));
    }

    #[test]
    fn test_spin_3() {
        let mut dance = Dance::new(5);
        dance.spin(3);
        assert_eq!(dance.programs, vec!('c', 'd', 'e', 'a', 'b'));
    }

    #[test]
    fn test_exchange() {
        let mut dance = Dance::new(5);
        dance.exchange(0, 2);
        assert_eq!(dance.programs, vec!('c', 'b', 'a', 'd', 'e'));
    }

    #[test]
    fn test_partner() {
        let mut dance = Dance::new(5);
        dance.partner('a', 'd');
        assert_eq!(dance.programs, vec!('d', 'b', 'c', 'a', 'e'));
    }
}

