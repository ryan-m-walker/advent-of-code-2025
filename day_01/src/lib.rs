pub(crate) fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
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

pub fn day_1(input: &str) -> (i32, i32) {
    let dirs = parse_input(input);

    let mut zero_count = 0;
    let mut zero_click_count = 0;
    let mut current = 50;

    for dir in dirs {
        for _ in 0..dir.abs() {
            if dir < 0 {
                current -= 1;
            } else {
                current += 1;
            }

            if current == 100 {
                current = 0;
            }

            if current == -1 {
                current = 99;
            }

            if current == 0 {
                zero_click_count += 1;
            }
        }

        if current == 0 {
            zero_count += 1;
        }
    }

    (zero_count, zero_click_count)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_test_input() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let (res, ..) = day_1(&input);
        assert_eq!(res, 3);
    }

    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let (res, ..) = day_1(&input);
        println!("[part_1] = {res}");
    }

    #[test]
    fn part_2_one_full_rotation_right_test() {
        let input = "R60";
        let (.., res) = day_1(input);
        assert_eq!(res, 1);
    }

    #[test]
    fn part_2_one_full_rotation_left_test() {
        let input = "L60";
        let (.., res) = day_1(input);
        assert_eq!(res, 1);
    }

    #[test]
    fn part_2_test_input() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let (.., res) = day_1(&input);
        assert_eq!(res, 6);
    }

    #[test]
    fn part_2_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let (.., res) = day_1(&input);
        println!("[part_2] = {res}");
    }
}
