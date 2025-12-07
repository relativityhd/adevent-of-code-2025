use clearscreen;
use std::{thread, time, vec};

advent_of_code::solution!(7);

fn print_map(input: &str, beam_state: &[bool], t: usize, n_splits: u64, n_new_splits: u64) {
    clearscreen::clear().expect("failed to clear screen");
    println!("T{t}: {n_splits} splits (+{n_new_splits})");
    for (i, line) in input.lines().enumerate() {
        if i == t + 1 {
            let mut linechars: Vec<char> = line.chars().collect();
            for (j, &state) in beam_state.iter().enumerate() {
                if state {
                    linechars[j] = '|';
                }
            }
            let newline: String = linechars.iter().collect();
            println!("{newline}")
        } else {
            println!("{line}");
        }
    }
}

fn print_map_two(input: &str, beam_state: &[u64], t: usize, n_splits: u64, n_new_splits: u64) {
    clearscreen::clear().expect("failed to clear screen");
    println!("T{t}: {n_splits} splits (+{n_new_splits})");
    for (i, line) in input.lines().enumerate() {
        if i == t + 1 {
            let mut linechars: Vec<char> = line.chars().collect();
            for (j, &state) in beam_state.iter().enumerate() {
                if state > 0 {
                    linechars[j] = char::from_digit(state as u32, 10).unwrap_or('X');
                }
            }
            let newline: String = linechars.iter().collect();
            println!("{newline}")
        } else {
            println!("{line}");
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let fieldsize = first_line.len();
    let tachyon_entrance_pos = first_line.chars().position(|c| c == 'S').unwrap();
    let mut beam_state = vec![false; fieldsize];
    beam_state[tachyon_entrance_pos] = true;
    let mut n_splits = 0;
    for (t, line) in lines.enumerate() {
        let mut n_new_splits = 0;
        for (i, c) in line.chars().enumerate() {
            if c == '^' && beam_state[i] {
                beam_state[i] = false;
                beam_state[(i + 1).min(fieldsize - 1)] = true;
                beam_state[i.saturating_sub(1)] = true;
                n_new_splits += 1;
            }
        }
        n_splits += n_new_splits;
        // print_map(input, &beam_state, t, n_splits, n_new_splits);
        // thread::sleep(time::Duration::from_secs(1));
    }
    Some(n_splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let fieldsize = first_line.len();
    let tachyon_entrance_pos = first_line.chars().position(|c| c == 'S').unwrap();
    let mut beam_state = vec![0u64; fieldsize];
    beam_state[tachyon_entrance_pos] = 1;
    let mut n_splits = 0;
    for (t, line) in lines.enumerate() {
        let mut n_new_splits = 0;
        for (i, c) in line.chars().enumerate() {
            if c == '^' && beam_state[i] > 0 {
                beam_state[(i + 1).min(fieldsize - 1)] += beam_state[i];
                beam_state[i.saturating_sub(1)] += beam_state[i];
                beam_state[i] = 0;
                n_new_splits += 1;
            }
        }
        n_splits += n_new_splits;
        // print_map_two(input, &beam_state, t, n_splits, n_new_splits);
        // thread::sleep(time::Duration::from_secs(1));
    }
    let res: u64 = beam_state.iter().sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
