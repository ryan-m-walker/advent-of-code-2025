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

pub fn day_2(input: &str) -> i64 {
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_test_input() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let output = day_2(&input);
        assert_eq!(output, 1227775554);
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = day_2(&input);
        println!("part 1 = {output}");
    }
}
