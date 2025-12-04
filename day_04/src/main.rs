use std::collections::HashSet;

pub fn part_1(input: &str) -> i32 {
    let mut count = 0;
    let mut rolls = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '@' {
                rolls.insert((x as i32, y as i32));
            }
        }
    }

    for (roll_x, roll_y) in &rolls {
        let mut roll_count = 0;

        for x in roll_x - 1..=roll_x + 1 {
            for y in roll_y - 1..=roll_y + 1 {
                if x == *roll_x && y == *roll_y {
                    continue;
                }

                let hit = rolls.get(&(x, y));

                if hit.is_some() {
                    roll_count += 1;
                }
            }
        }

        if roll_count < 4 {
            count += 1;
        }
    }

    count
}

pub fn part_2(input: &str) -> i32 {
    let mut total = 0;
    let mut rolls: HashSet<(i32, i32)> = HashSet::new();
    let mut next_rolls: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '@' {
                rolls.insert((x as i32, y as i32));
            }
        }
    }

    loop {
        let mut count = 0;

        for (roll_x, roll_y) in &rolls {
            let mut neighbor_count = 0;

            for x in roll_x - 1..=roll_x + 1 {
                for y in roll_y - 1..=roll_y + 1 {
                    if x == *roll_x && y == *roll_y {
                        continue;
                    }

                    let hit = rolls.get(&(x, y));

                    if hit.is_some() {
                        neighbor_count += 1;
                    }
                }
            }

            if neighbor_count < 4 {
                count += 1;
            } else {
                next_rolls.insert((*roll_x, *roll_y));
            }
        }

        total += count;

        if count == 0 {
            break;
        }

        rolls = next_rolls;
        next_rolls = HashSet::new();
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
        let output = part_1(&input);
        assert_eq!(output, 13);
    }

    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        println!("part 1 = {output}");
    }

    #[test]
    fn part_2_test_input() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 43);
    }

    #[test]
    fn part_2_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        println!("part 2 = {output}");
    }
}
