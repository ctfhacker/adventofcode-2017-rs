#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Register {
    name: String,
    value: isize
}

impl Register {
    fn new(name: String) -> Register {
        Register {
            name: name,
            value: 0
        }
    }

    fn inc(&mut self, value: isize) {
        self.value += value;
    }

    fn dec(&mut self, value: isize) {
        self.value -= value;
    }
}

#[derive(Debug)]
struct Machine {
    registers: HashMap<String, Register>,
    largest_ever_value: isize
}

impl Machine {
    fn new() -> Machine {
        Machine {
            registers: HashMap::new(),
            largest_ever_value: -9999999
        }
    }

    fn new_register(&mut self, name: String) {
        self.registers.insert(name.clone(), Register::new(name));
    }

    fn parse_line(&mut self, line: &str) {
        lazy_static! {
            static ref LINE_REGEX: Regex = Regex::new("([a-z]*) (inc|dec) (.*) if ([a-z]*) (.*) (.*)").unwrap();
        }

        for matches in LINE_REGEX.captures_iter(line) {
            {
                let conditional_register = matches.get(4).expect("Unable to retrieve conditional register").as_str();
                let conditional_operation = matches.get(5).expect("Unable to retrieve conditional operation").as_str();
                let conditional_value = matches.get(6).expect("Unable to retrieve conditional value")
                                                      .as_str().parse::<isize>().ok().expect("Conditional value cannot parse to i32");

                if self.registers.get(conditional_register).is_none() { self.new_register(conditional_register.to_string()); }
                let curr_register = self.registers.get(conditional_register).expect("Wanted register is not found in register hashmap");

                match conditional_operation {
                    "==" => if curr_register.value != conditional_value { continue; },
                    "<"  => if curr_register.value >= conditional_value { continue; },
                    "<=" => if curr_register.value >  conditional_value { continue; },
                    ">"  => if curr_register.value <= conditional_value { continue; },
                    ">=" => if curr_register.value <  conditional_value { continue; },
                    "!=" => if curr_register.value == conditional_value { continue; },
                    _ => unimplemented!()
                }
            }

            let result_register = matches.get(1).expect("Unable to retrieve result register").as_str();
            let result_operation = matches.get(2).expect("Unable to retrieve result operation").as_str();
            let result_value = matches.get(3).expect("Unable to retrieve result value")
                                             .as_str().parse::<isize>().ok().expect("result value cannot parse to i32");

            if self.registers.get(result_register).is_none() { self.new_register(result_register.to_string()); }
            let mut curr_register = self.registers.get_mut(result_register).expect("Result register is not found in register hashmap");
            
            match result_operation {
                "inc" => { curr_register.inc(result_value); },
                "dec" => { curr_register.dec(result_value); },
                _ => panic!("Invalid operation received..")
            }

            // Task 2 - Keep track of largest value ever present in the machine
            if curr_register.value > self.largest_ever_value {
                self.largest_ever_value = curr_register.value;
            }
        }
    }

    /// Task 1 - Largest value at the end of execution
    fn largest_value(&self) -> isize {
        let mut largest_value = -999999;

        for (name, reg) in self.registers.iter() {
            if reg.value < largest_value { continue; }
            largest_value = reg.value;
        }

        largest_value
    }

    /// Task 2 - Largest value ever had in the machine
    fn largest_ever_value(&self) -> isize {
        self.largest_ever_value
    }
}

fn task_1() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");

    let mut machine = Machine::new();

    for line in contents.lines() {
        machine.parse_line(line);
    }

    println!("Largest current value: {:?}", machine.largest_value());
    println!("Largest ever value:    {:?}", machine.largest_ever_value());
}


fn main() {
    task_1();
}
