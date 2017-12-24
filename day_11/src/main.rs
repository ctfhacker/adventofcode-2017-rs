use std::fs::File;
use std::io::Read;

fn task_1() {
    let mut f = File::open("./input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");
    let mut coords = Coords::new();
    let origin = Coords::new();
    coords.hex_move(&contents);
    let result = coords.distance(origin);
    println!("Result task 1: {}", result);
}

struct Coords {
    x: isize,
    y: isize,
    z: isize
}

impl Coords {
    fn new() -> Coords {
        Coords { x: 0, y: 0, z: 0 }
    }

    fn new_from_point(x: isize, y: isize, z: isize) -> Coords {
        Coords {
            x: x,
            y: y,
            z: z,
        }
    }

    fn get(&self) -> (isize, isize, isize) {
        (self.x, self.y, self.z)
    }

    fn distance(&self, other: Coords) -> isize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }

    fn hex_move(&mut self, directions: &str) {
        /* https://www.redblobgames.com/grids/hexagons/ */

        let mut furthest = 0;

        for d in directions.trim().split(",") {
            match d {
                "ne" => { self.x += 1; self.z -= 1; },
                "n"  => { self.y += 1; self.z -= 1; },
                "nw" => { self.y += 1; self.x -= 1; },
                "sw" => { self.z += 1; self.x -= 1; },
                "s"  => { self.z += 1; self.y -= 1; },
                "se" => { self.x += 1; self.y -= 1; },
                _ => { println!("{}", d); panic!("Recieved invalid hex direction") }
            }

            let dist_from_origin = self.distance(Coords::new()) ;
            if dist_from_origin > furthest {
                furthest = dist_from_origin;
            }
        }

        println!("Furthest distance from origin: {}", furthest);
    }
}


fn main() {
    task_1();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_1() {
        let mut coords = Coords::new();
        let origin = Coords::new();
        coords.hex_move("ne,ne,ne");
        let result = coords.distance(origin);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_move_back_to_start() {
        let mut coords = Coords::new();
        let origin = Coords::new();
        coords.hex_move("ne,ne,sw,sw");
        let result = coords.distance(origin);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_move_2() {
        let mut coords = Coords::new();
        let origin = Coords::new();
        coords.hex_move("ne,ne,s,s");
        let result = coords.distance(origin);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_move_3() {
        let mut coords = Coords::new();
        let origin = Coords::new();
        coords.hex_move("se,sw,se,sw,sw");
        let result = coords.distance(origin);
        assert_eq!(result, 3);
    }
}
