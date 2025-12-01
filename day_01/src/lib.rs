use std::fs;

#[derive(Debug)]
pub(crate) enum Dir {
    Left(i32),
    Right(i32),
}

pub(crate) fn parse_input(path: &str) -> Vec<Dir> {
    let input = fs::read_to_string(path).unwrap();

    input
        .lines()
        .map(|line| {
            let dir = &line[..1];
            let amount = &line[1..];

            match dir {
                "L" => Dir::Left(amount.parse().unwrap()),
                "R" => Dir::Right(amount.parse().unwrap()),
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
        current = match dir {
            Dir::Left(amount) => {
                let result = current - (amount % 100);
                if result < 0 { 100 + result } else { result }
            }
            Dir::Right(amount) => {
                let result = current + (amount % 100);
                if result > 99 { result % 100 } else { result }
            }
        };

        if current == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn part_2(input: &str) -> i32 {
    let dirs = parse_input(input);

    let mut zero_count = 0;
    let mut current = 50;

    for dir in dirs {
        current = match dir {
            Dir::Left(amount) => {
                let result = current - (amount % 100);
                if result < 0 { 100 + result } else { result }
            }
            Dir::Right(amount) => {
                let result = current + (amount % 100);
                if result > 99 { result % 100 } else { result }
            }
        };

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

    // 281
    #[test]
    fn part_1_test() {
        let res = part_1("./input.txt");
        println!("part_1 test output = {res}");
    }
}
