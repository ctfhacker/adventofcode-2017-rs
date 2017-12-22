use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Maze {
    path: Vec<i32>,
    position: i32,
    steps: u32
}

impl Maze {
    fn new(path: Vec<i32>) -> Maze {
        Maze {
            path: path,
            position: 0,
            steps: 0
        }
    }

    fn step(&mut self) -> bool {
        let orig_number;
        if let Some(num) = self.path.iter_mut().nth(self.position.abs() as usize) {
            orig_number = num.clone();
            /* Task 1 */
            // *num += 1;
            
            /* Task 2 */
            if orig_number >= 3 {
                *num -= 1;
            } else {
                *num += 1;
            }
        } else {
            return false;
        }
        self.position += orig_number;
        self.steps += 1;
        true
    }
}

fn task() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");

    let numbers = contents.trim()
                          .lines()
                          .filter_map(|n| n.parse::<i32>().ok())
                          .collect::<Vec<i32>>();

    let mut maze = Maze::new(numbers);

    loop {
        if !maze.step() {
            break
        }
    }
    println!("Task 2: {:?}", maze);
}

fn main() {
    task();
}
