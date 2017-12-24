fn twist(mut numbers: Vec<usize>, lengths: Vec<usize>, rounds: usize) -> Vec<usize>{
    let mut index = 0;
    let mut skip = 0;
    let numbers_length = numbers.len();
    for _ in 0..rounds {
        for curr_length in lengths.clone() {
            for x in 0..curr_length/2 {
                numbers.swap((index + x) % numbers_length, (index + curr_length - 1 - x) % numbers_length);
            }
            index = (index + skip + curr_length) % numbers_length;
            skip += 1;
        }
    }
    numbers
}

fn main() {
    let mut input: Vec<_> = "192,69,168,160,78,1,166,28,0,83,198,2,254,255,41,12"
                                .bytes()
                                .map(|b| b as usize)
                                .collect();

    input.extend(vec!(17, 31, 73, 47, 23));

    let twisted = twist((0..256).collect(), input, 64);
    let result = twisted.chunks(16)   // Chunk the vec of 256 elements into vec of 16 16-sized elements
                        .map(|c| c.iter().fold(0, |acc, &x| acc ^ x)) // xor all elements of each chunk together
                        .map(|n| format!("{:02x}", n).to_string()) // 0 left pad hex convert each digit
                        .collect::<Vec<_>>() // Convert to a Vector
                        .join(""); // Join with no spaces

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_lengths() {
        assert_eq!(twist(vec!(0, 1, 2, 3, 4), vec!(3), 1), vec!(2, 1, 0, 3, 4));
    }

    #[test]
    fn test_two_lengths() {
        assert_eq!(twist(vec!(0, 1, 2, 3, 4), vec!(3, 4), 1), vec!(4, 3, 0, 1, 2));
    }

    #[test]
    fn test_three_lengths() {
        assert_eq!(twist(vec!(0, 1, 2, 3, 4), vec!(3, 4, 1), 1), vec!(4, 3, 0, 1, 2));
    }

    #[test]
    fn test_four_lengths() {
        assert_eq!(twist(vec!(0, 1, 2, 3, 4), vec!(3, 4, 1, 5), 1), vec!(3, 4, 2, 1, 0));
    }

    #[test]
    fn test_string_one() {
        let mut input: Vec<_> = "1,2,3".bytes()
                                    .map(|b| b as usize)
                                    .collect();

        input.extend(vec!(17, 31, 73, 47, 23));

        let twisted = twist((0..256).collect(), input, 64);
        let result = twisted.chunks(16)
                                    .map(|c| c.iter().fold(0, |acc, &x| acc ^ x))
                                    .map(|n| format!("{:02x}", n).to_string())
                                    .collect::<Vec<_>>()                  
                                    .join("");

        assert_eq!(result, "3efbe78a8d82f29979031a4aa0b16a9d");
    }
}
