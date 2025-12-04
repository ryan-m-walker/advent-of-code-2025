fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_1(input: &str) {
    let banks = parse_input(input);

    for bank in banks {
        let len = bank.len();

        let mut a = 0;
        let mut a_idx = 0;

        let mut b = 0;
        let mut b_idx = len;

        for n in 0..len {
            let cur_a = bank[n];
            let a_dig = cur_a.to_digit(10).unwrap();

            let cur_b = bank[len - 1];
            let b_dig = cur_b.to_digit(10).unwrap();

            if a_dig > a {
                a = a_dig;
                a_idx = n;
            }

            if b_dig > b {
                b = b_dig;
                b_idx = len - 1;
            }
        }

        println!("{a}{b}");
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        // let input = String::from("111189222");
        let output = part_1(&input);
    }
}
