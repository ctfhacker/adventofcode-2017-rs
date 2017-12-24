use std::fs::File;
use std::io::Read;

fn task_1() {
    let mut f = File::open("./input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");
    let firewall = Firewall::new_from_input(&contents);
    for i in 0.. {
        if firewall.traverse(i) {
            println!("Task 2 results: {}", i);
            break
        }
    }
}

#[derive(Debug, Clone)]
struct Layer {
    depth: usize,
    range: usize
}

impl Layer {
    fn new(depth: usize, range: usize) -> Layer {
        Layer {
            depth: depth,
            range: range
        }
    }

    fn caught(&self, delay: usize) -> bool {
        (self.depth + delay) % (2 * (self.range-1)) == 0
    }
}

#[derive(Debug, Clone)]
struct Firewall {
    layers: Vec<Layer>
}

impl Firewall {
    fn new_from_input(input: &str) -> Firewall {
        let mut layers = Vec::new();

        for line in input.trim().lines() {
            let layer: Vec<_> = line.trim()
                                    .split(": ")
                                    .map(|d| d.parse().ok().unwrap())
                                    .collect();

            let (depth, range) = (layer[0], layer[1]);
            layers.push(Layer::new(depth, range));
        }

        Firewall {
            layers: layers
        }
    }

    fn traverse(&self, delay: usize) -> bool {
        !self.layers.iter().any(|l| l.caught(delay))
    }
}

fn main() {
    task_1();
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_firewalls() {
        let INPUT = "0: 3
        1: 2
        4: 4
        6: 4";
        let mut firewall = Firewall::new_from_input(INPUT);
        assert_eq!(firewall.traverse(), 24);
    }
    */
}
    #[test]
    fn test_delay() {
        let INPUT = "0: 3
        1: 2
        4: 4
        6: 4";
        let firewall = Firewall::new_from_input(INPUT);
        for i in 0..100 {
            if firewall.traverse(i) {
                assert_eq!(i, 10);
                break
            }
        }
    }
