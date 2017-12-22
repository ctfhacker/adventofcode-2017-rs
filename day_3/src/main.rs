use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug)]
struct Marker {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    direction: Direction,
    coord: (isize, isize),
    grid: HashMap<(isize, isize), usize>
}

impl Marker {
    fn new() -> Marker {
        let mut grid = HashMap::new();
        grid.insert((0, 0), 1);

        Marker {
            left: 2,
            right: 1,
            up: 1,
            down: 2,
            direction: Direction::Right,
            coord: (0, 0),
            grid: grid
        }
    }

    /// Simple state machine to step the marker in a spiral shape as described in the problem.
    fn step(&mut self) {
        match self.direction {
            Direction::Left => {
                self.coord.0 -= 1;
                self.left -= 1;
                if self.left == 0 {
                    self.left = self.right + 1;
                    self.direction = Direction::Down;
                }
            },
            Direction::Right => {
                self.coord.0 += 1;
                self.right -= 1;
                if self.right == 0 {
                    self.right = self.left + 1;
                    self.direction = Direction::Up;
                }
            },
            Direction::Up => {
                self.coord.1 += 1;
                self.up -= 1;
                if self.up == 0 {
                    self.up = self.down + 1;
                    self.direction = Direction::Left;
                }
            },
            Direction::Down => {
                self.coord.1 -= 1;
                self.down -= 1;
                if self.down == 0 {
                    self.down = self.up + 1;
                    self.direction = Direction::Right;
                }
            }
        }

        // Write the sum of surrounding squares by adding together the current valid squares
        // surrounding the marker
        let mut sum = 0;
        for (x_offset, y_offset) in vec!((-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0)) {
            if let Some(value) = self.grid.get(&(self.coord.0+x_offset, self.coord.1+y_offset)) {
                sum += value;
            }
        }

        self.grid.insert(self.coord, sum);
    }

    fn coord(&self) -> Option<(isize, isize)> {
        Some(self.coord)
    }

    /// Get the current marker's distance from the origin (used in Task 1)
    fn distance(&self) -> isize {
        self.coord.0.abs() + self.coord.1.abs()
    }
}

fn main() {
    let mut marker = Marker::new();
    /*
    for i in 2..20 {
        marker.step();
        println!("{} -- {:?} -- {}", i, marker.coord, marker.grid.get(&marker.coord).unwrap());
    }
    */
    for i in 2..325490 {
        marker.step();
        if let Some(value) = marker.grid.get(&marker.coord) {
            if value < &325489 { continue; }
            println!("{} -- {:?} -- {}", i, marker.coord, marker.grid.get(&marker.coord).unwrap());
            break;
        }
    }
}
