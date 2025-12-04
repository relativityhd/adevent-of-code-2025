advent_of_code::solution!(4);

fn parse_map(input: &str) -> Vec<Vec<u8>> {
    input
        .strip_suffix("\n")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => 1,
                    '.' => 0,
                    _ => 2,
                })
                .collect()
        })
        .collect()
}

fn convolve(map: &Vec<Vec<u8>>, kernel: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut out = Vec::<Vec<u8>>::with_capacity(map.len());
    for i in 0..map.len() {
        out.insert(i, Vec::<u8>::with_capacity(map[i].len()));
        for j in 0..map[i].len() {
            out[i].insert(j, 0);
            for k in 0..kernel.len() {
                let k_offset = (k as isize) - (kernel.len().div_euclid(2) as isize);
                let ipx = i.saturating_sub_signed(k_offset).min(map.len() - 1);
                if (ipx as isize) + k_offset != (i as isize) {
                    continue;
                }
                for l in 0..kernel[k].len() {
                    let v = kernel[k][l];
                    if v == 0 {
                        continue;
                    }
                    let l_offset = (l as isize) - (kernel[k].len().div_euclid(2) as isize);
                    let jpx = j.saturating_sub_signed(l_offset).min(map[i].len() - 1);
                    if (jpx as isize) + l_offset != (j as isize) {
                        continue;
                    }
                    out[i][j] += map[ipx][jpx] * v;
                }
            }
        }
    }
    out
}

fn reachable(map: &Vec<Vec<u8>>, convolved: &Vec<Vec<u8>>) -> usize {
    map.iter()
        .flatten()
        .zip(convolved.iter().flatten())
        .map(|(&v, &c)| (c <= 3) as usize * v as usize)
        .sum()
}

fn update_map(map: &mut Vec<Vec<u8>>, convolved: &Vec<Vec<u8>>) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if convolved[i][j] <= 3 && map[i][j] == 1 {
                map[i][j] = 0;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_map(input);
    let kernel = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let convolved = convolve(&map, &kernel);
    let res = reachable(&map, &convolved);
    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = parse_map(input);
    let kernel = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let mut total = 0;
    loop {
        let convolved = convolve(&map, &kernel);
        let r = reachable(&map, &convolved);
        total += r;
        if r == 0 {
            break;
        }
        update_map(&mut map, &convolved);
    }
    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
