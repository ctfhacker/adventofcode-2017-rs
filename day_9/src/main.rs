fn score(mut stream: &str) -> (usize, usize) {
    let mut score = 0;
    let mut group_level = 0;
    let mut garbage = false;
    let mut chars = stream.chars();
    let mut num_garbage = 0;
    let mut i = 0;
    while let Some(ch) = chars.next() {
        match ch {
            '!' => { chars.next(); },
            '{' => { if !garbage { group_level += 1; } else { num_garbage += 1; } },
            '<' => { if garbage { num_garbage += 1; } garbage = true; },
            '}' => { if !garbage { score += group_level; group_level -= 1; } else { num_garbage += 1; } },
            '>' => { garbage = false; },
            _ => { if garbage { num_garbage += 1; } }
        }
        i += 1;
        println!("[{}], {}", num_garbage, ch);
    }
    (score, num_garbage)
}

fn main() {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open("./input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");
    let (score, garbage) = score(contents.trim());
    println!("Result task 1: {} -- {}", score, garbage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_groups() {
        assert_eq!(score("{}").0, 1);
    }

    #[test]
    fn test_three_groups() {
        assert_eq!(score("{{},{}}").0, 5);
    }

    #[test]
    fn test_three_groups_2() {
        assert_eq!(score("{{{}}}").0, 6);
    }

    #[test]
    fn test_larget_group() {
        assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
    }

    #[test]
    fn test_garbage_1() {
        assert_eq!(score("<random characters>").1, 17);
    }

    #[test]
    fn test_garbage_2() {
        assert_eq!(score("<{o\"i!a,<{i<a>").1, 10);
    }
}

