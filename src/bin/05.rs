use std::{
    ops::{Range, RangeBounds},
    vec,
};

advent_of_code::solution!(5);

fn parse_freshness(input: &str) -> Vec<Range<usize>> {
    input
        .lines()
        .map(|line| {
            let a: Vec<usize> = line.split('-').map(|s| s.parse().unwrap()).collect();
            a[0]..a[1] + 1
        })
        .collect()
}

fn parse_ingredians(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn parse_database(input: &str) -> (Vec<Range<usize>>, Vec<usize>) {
    let a: Vec<&str> = input.split("\n\n").collect();
    let freshness = parse_freshness(a[0]);
    let ingredians = parse_ingredians(a[1].strip_suffix('\n').unwrap());
    (freshness, ingredians)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (freshness, ingredians) = parse_database(input);

    let mut n_fresh = 0;

    for i in ingredians {
        for f in &freshness {
            if f.contains(&i) {
                n_fresh += 1;
                break;
            }
        }
    }

    Some(n_fresh)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut freshness, _) = parse_database(input);

    // Sort freshness ranges by start
    freshness.sort_by(|a, b| a.start.cmp(&b.start));

    let mut fresh_ranges: Vec<Range<usize>> = vec![];

    for i in 0..freshness.len() {
        let f = freshness.get(i).unwrap();
        if fresh_ranges.is_empty() {
            fresh_ranges.push(f.clone());
            continue;
        }

        // If f partially overlaps with range before, change the end
        let lastf = fresh_ranges.last_mut().unwrap();
        if lastf.contains(&f.start) {
            lastf.end = f.end.max(lastf.end);
        } else {
            fresh_ranges.push(f.clone());
        }
    }

    let mut total = 0;
    for f in fresh_ranges {
        total += f.end - f.start;
    }

    Some(total as u64)
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
        assert_eq!(result, Some(14));
    }
}
