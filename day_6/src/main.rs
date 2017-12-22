use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct Memory {
    banks: [u32; 16],
}

impl Memory {
    fn new(banks: [u32; 16]) -> Memory {
        Memory { banks: banks }
    }

    fn redistribute(&mut self) {
        let max_num = self.banks.iter().max().expect("Can't find max value").clone();
        let max_position = self.banks.iter().position(|&x| x == max_num).expect("Did not find value in memory");
        self.banks[max_position] = 0;

        let mut curr_position = (max_position + 1) % 16;
        for _i in 0..max_num {
            self.banks[curr_position] += 1;
            curr_position = (curr_position + 1) % 16;
        }
    }

    fn banks(&self) -> [u32; 16] {
        self.banks
    }
}

fn task_1() {
    /* 
     * Strategy:
     * Insert the state of the memory after each redistribution into a HashSet. Once we can't insert a state
     * into the HashSet (because it already exists), we know that we have found a set that we have
     * seen before
     */
    let mut mem = Memory::new([10, 3, 15, 10, 5, 15, 5, 15, 9, 2, 5, 8, 5, 2, 3, 6]);
    let mut states = HashSet::new();
    println!("Mem: {:?}", mem);
    for i in 1..9999999999 {
        mem.redistribute();
        if !states.insert(mem.banks()) {
            println!("Task 1: {}", i);
            break;
        }
    }
}

fn task_2() {
    /* 
     * Strategy: 
     * While keeping track of seen states in a HashSet, we also keep track of which round we saw
     * each state in a HashMap. Once we see that a state cannot be inserted into the HashSet, we
     * query the HashMap for the original round it was seen and then print the different between
     * that round and the current round
     */

    let mut mem = Memory::new([10, 3, 15, 10, 5, 15, 5, 15, 9, 2, 5, 8, 5, 2, 3, 6]);
    let mut states = HashSet::new();
    let mut rounds = HashMap::new();
    println!("Mem: {:?}", mem);
    for i in 1..9999999999 {
        mem.redistribute();
        if !states.insert(mem.banks()) {
            if let Some(first_seen) = rounds.get(&mem.banks()) {
                let round_diff = i - first_seen;
                println!("Task 2: {}", round_diff);
                break;
            }
        }
        rounds.insert(mem.banks(), i);
    }
}

fn main() {
    task_1();
    task_2();
}
