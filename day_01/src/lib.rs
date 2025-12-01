use std::fs;

pub(crate) fn parse_input(path: &str) -> Vec<i32> {
    let input = fs::read_to_string(path).unwrap();

    input
        .lines()
        .map(|line| {
            let dir = &line[..1];
            let amount = &line[1..];

            match dir {
                "L" => -(amount.parse::<i32>().unwrap()),
                "R" => amount.parse::<i32>().unwrap(),
                _ => panic!("unexpected dir"),
            }
        })
        .collect()
}

pub fn part_1(input: &str) -> i32 {
    let dirs = parse_input(input);

    let mut zero_count = 0;
    let mut current = 50;

    for dir in dirs {
        let diff = (dir + current) % 100;

        if diff < 0 {
            current = 100 + diff;
        } else {
            current = diff;
        }

        if current == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_input() {
        let res = part_1("./test_input.txt");
        assert_eq!(res, 3);
    }

    #[test]
    fn part_1_test() {
        let res = part_1("./input.txt");
        println!("part_1 test output = {res}");
    }
}
