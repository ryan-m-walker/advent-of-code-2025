fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .split(",")
        .map(|range| {
            let split: Vec<&str> = range.split("-").collect();
            let lower = split[0].trim().parse().unwrap();
            let upper = split[1].trim().parse().unwrap();
            (lower, upper)
        })
        .collect()
}

pub fn part_1(input: &str) -> i64 {
    let ranges = parse_input(input);
    let mut total = 0;

    for (lower, upper) in ranges {
        for n in lower..=upper {
            let n_str = n.to_string();

            if n_str.len() % 2 != 0 {
                continue;
            }

            let (lh, rh) = n_str.split_at(n_str.len() / 2);

            if lh == rh {
                total += n;
            }
        }
    }

    total
}

pub fn part_2(input: &str) -> i64 {
    let ranges = parse_input(input);
    let mut total = 0;

    for (lower, upper) in ranges {
        'outer: for n in lower..=upper {
            let n_str = n.to_string();

            'inner: for i in 1..n_str.len() {
                let chunks = n_str.as_bytes().chunks(i);

                let mut first = None;

                for (i, chunk) in chunks.enumerate() {
                    if i == 0 {
                        first = Some(chunk);
                        continue;
                    }

                    let Some(first) = first else {
                        continue;
                    };

                    if first != chunk {
                        continue 'inner;
                    }
                }

                total += n_str.parse::<i64>().unwrap();
                continue 'outer;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_test_input_test() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 1227775554);
    }

    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        println!("part 1 = {output}");
    }

    #[test]
    fn part_2_test_input_test() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 4174379265);
    }

    #[test]
    fn parse_2_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        println!("part 2 = {output}");
    }
}
