advent_of_code::solution!(1);

#[derive(Debug)]
enum Rotation {
    R(isize),
    L(isize),
}

fn parse_rotations(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let distance = distance.parse().unwrap();
            if direction == "R" {
                Rotation::R(distance)
            } else {
                Rotation::L(distance)
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let rotations = parse_rotations(input);
    let mut dial = 50;
    let mut nulls = 0;
    for r in rotations {
        dial += match r {
            Rotation::L(d) => -d,
            Rotation::R(d) => d,
        };
        while dial < 0 {
            dial += 100;
        }
        while dial >= 100 {
            dial -= 100;
        }
        if dial == 0 {
            nulls += 1;
        }
    }
    Some(nulls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations = parse_rotations(input);
    let mut dial = 50;
    let mut nulls = 0;
    for r in rotations {
        dial = match r {
            Rotation::L(d) => {
                let mut new_dial = dial - d;
                // Handle exact zero crossings
                if dial == 0 && new_dial < 0 {
                    nulls -= 1;
                }
                while new_dial < 0 {
                    new_dial += 100;
                    nulls += 1;
                }
                if new_dial == 0 {
                    nulls += 1;
                }
                new_dial
            }
            Rotation::R(d) => {
                let mut new_dial = dial + d;
                while new_dial >= 100 {
                    new_dial -= 100;
                    nulls += 1;
                }
                new_dial
            }
        };
    }
    Some(nulls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_edges() {
        let input = "L50\nR50\n";
        let result = part_two(input);
        assert_eq!(result, Some(1));

        let input = "R50\nL50\n";
        let result = part_two(input);
        assert_eq!(result, Some(1));

        let input = "L50\nL50\n";
        let result = part_two(input);
        assert_eq!(result, Some(1));

        let input = "R50\nR50\n";
        let result = part_two(input);
        assert_eq!(result, Some(1));

        let input = "L150\nL50\n";
        let result = part_two(input);
        assert_eq!(result, Some(2));

        let input = "L150\nR50\n";
        let result = part_two(input);
        assert_eq!(result, Some(2));

        let input = "R150\nL50\n";
        let result = part_two(input);
        assert_eq!(result, Some(2));

        let input = "R150\nR50\n";
        let result = part_two(input);
        assert_eq!(result, Some(2));
    }
}
