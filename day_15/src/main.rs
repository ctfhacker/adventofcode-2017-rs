#![feature(nll)]

struct Generator {
    value: u64,
    factor: u64
}

impl Generator {
    fn new(value: u64, factor: u64) -> Generator {
        let mut new_gen = Generator { value: value, factor: factor };
        new_gen.next();
        new_gen
    }

    fn value(&self) -> u64 {
        self.value
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.value = (self.value * self.factor) % 2147483647;
        Some(self.value)
    }

}

fn main() {
    let mut gen_a = Generator::new(512, 16807);
    let mut gen_b = Generator::new(191, 48271);

    let mut total = 0;
    for _i in 0..5000000 {
        while (gen_a.value() % 4) != 0 {
            gen_a.next();
        }

        while (gen_b.value() % 8) != 0 {
            gen_b.next();
        }

        if gen_a.value() & 0xffff == gen_b.value() & 0xffff {
            total += 1;
        }

        gen_a.next(); gen_b.next();
    }

    println!("Total: {}", total);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut gen_a = Generator::new(65, 16807);
        let mut gen_b = Generator::new(8921, 48271);

        assert_eq!(gen_a.value(), 1092455);
        assert_eq!(gen_b.value(), 430625591);
    }

    #[test]
    fn test_task_2() {
        let mut gen_a = Generator::new(65, 16807);
        let mut gen_b = Generator::new(8921, 48271);
        let mut total = 0;
        for _i in 0..5000000 {
            while (gen_a.value() % 4) != 0 {
                gen_a.next();
            }

            while (gen_b.value() % 8) != 0 {
                gen_b.next();
            }

            if gen_a.value() & 0xffff == gen_b.value() & 0xffff {
                total += 1;
            }

            gen_a.next(); gen_b.next();
        }
        assert_eq!(total, 309);
    }
}
