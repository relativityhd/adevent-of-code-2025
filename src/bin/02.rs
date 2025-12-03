use std::ops::Range;

advent_of_code::solution!(2);

fn parse_ranges(input: &str) -> Vec<Range<usize>> {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split(",")
        .map(|part| {
            let mut bounds = part.split('-').map(|n| n.parse::<usize>().unwrap());
            let start = bounds.next().unwrap();
            let end = bounds.next().unwrap();
            start..end + 1
        })
        .collect()
}

fn is_invalid_twice(num: usize) -> bool {
    let digits = num.to_string().chars().collect::<Vec<char>>();
    if digits.len() % 2 != 0 {
        return false;
    }
    let half_size = digits.len() / 2;
    for i in 0..half_size {
        if digits[i] != digits[i + half_size] {
            return false;
        }
    }
    return true;
}

fn is_invalid(num: usize) -> bool {
    let digits = num.to_string().chars().collect::<Vec<char>>();
    for chunk_size in 1..(digits.len() / 2 + 1) {
        if digits.len() % chunk_size != 0 {
            continue;
        }
        // Now it is assured that the chunk_size is valid and there are at least 2 chunks
        let n_chunks = digits.len() / chunk_size;
        let mut all_same = true;
        'chunkloop: for chunk in 1..n_chunks {
            for j in 0..chunk_size {
                if digits[j] != digits[j + chunk * chunk_size] {
                    all_same = false;
                    break 'chunkloop;
                }
            }
        }
        if all_same {
            return true;
        }
    }
    return false;
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);
    let mut total_invalids = 0;
    for range in ranges {
        for num in range {
            if is_invalid_twice(num) {
                total_invalids += num;
            }
        }
    }
    return Some(total_invalids as u64);
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);
    let mut total_invalids = 0;
    for range in ranges {
        for num in range {
            if is_invalid(num) {
                total_invalids += num;
            }
        }
    }
    return Some(total_invalids as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
