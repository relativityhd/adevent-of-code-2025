use std::{str::FromStr, vec};

advent_of_code::solution!(6);

enum Operator {
    Add,
    Mul,
}

#[derive(Debug)]
struct InvalidError;

impl FromStr for Operator {
    type Err = InvalidError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "*" => Ok(Operator::Mul),
            "+" => Ok(Operator::Add),
            _ => Err(InvalidError),
        }
    }
}

struct Problem {
    op: Operator,
    nums: Vec<u64>,
}

impl Problem {
    fn empty_from_op(op: Operator) -> Self {
        Problem { op, nums: vec![] }
    }
}

fn parse_problems(input: &str) -> Vec<Problem> {
    let mut problems: Vec<Problem> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|opc| Problem::empty_from_op(opc.parse().unwrap()))
        .collect();

    for line in input.lines().take(input.lines().count() - 1) {
        for (numc, problem) in line.split_whitespace().zip(problems.iter_mut()) {
            problem.nums.push(numc.parse().unwrap())
        }
    }

    problems
}

// Source - https://stackoverflow.com/a
// Posted by Netwave, modified by community. See post 'Timeline' for change history
// Retrieved 2025-12-07, License - CC BY-SA 4.0

fn transpose2<T: Copy>(rows: Vec<Vec<T>>, pad: T) -> Vec<Vec<T>> {
    assert!(!rows.is_empty());
    let len = rows.iter().map(|col| col.len()).max().unwrap();

    let mut iters: Vec<_> = rows.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap_or(pad))
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Symbol {
    Add,
    Mul,
    Num(u32),
    Empty,
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '*' => Symbol::Mul,
            '+' => Symbol::Add,
            _ => value.to_digit(10).map_or(Symbol::Empty, |v| Symbol::Num(v)),
        }
    }
}

impl Into<char> for &Symbol {
    fn into(self) -> char {
        match self {
            Symbol::Add => '+',
            Symbol::Mul => '*',
            Symbol::Num(v) => char::from_digit(*v, 10).unwrap(),
            Symbol::Empty => ' ',
        }
    }
}

fn parse_cephalopods(input: &str) -> Vec<Problem> {
    let rows: Vec<Vec<Symbol>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .collect();
    let cols = transpose2(rows, Symbol::Empty);

    let mut problems: Vec<Problem> = vec![];
    for col in cols {
        match col.last().unwrap() {
            Symbol::Add => {
                problems.push(Problem::empty_from_op(Operator::Add));
                Ok(())
            }
            Symbol::Mul => {
                problems.push(Problem::empty_from_op(Operator::Mul));
                Ok(())
            }
            Symbol::Num(_) => Err(InvalidError),
            Symbol::Empty => Ok(()),
        }
        .unwrap();
        let n_digits = col.len() - 1;
        let numstring = col
            .iter()
            .take(n_digits)
            .map(|s| <&Symbol as Into<char>>::into(s))
            .collect::<String>();
        let number = numstring.trim().parse();
        if let Ok(n) = number {
            problems.last_mut().unwrap().nums.push(n);
        }
    }
    problems
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse_problems(input);
    let mut results_total = 0;
    for problem in problems {
        let res = match problem.op {
            Operator::Add => problem.nums.iter().fold(0, |a, b| a + b),
            Operator::Mul => problem.nums.iter().fold(1, |a, b| a * b),
        };
        results_total += res;
    }

    Some(results_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = parse_cephalopods(input);
    let mut results_total = 0;
    for problem in problems {
        let res = match problem.op {
            Operator::Add => problem.nums.iter().fold(0, |a, b| a + b),
            Operator::Mul => problem.nums.iter().fold(1, |a, b| a * b),
        };
        results_total += res;
    }

    Some(results_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
