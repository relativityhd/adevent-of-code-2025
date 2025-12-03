advent_of_code::solution!(3);

fn parse_digits(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn max_joltage(batteries: &Vec<usize>) -> usize {
    let mut first_battery = batteries[0];
    let mut second_battery = batteries[1];
    for i in 1..batteries.len() {
        let battery = batteries[i];
        if battery > first_battery && i != batteries.len() - 1 {
            first_battery = battery;
            second_battery = batteries[i + 1]
        } else if battery > second_battery {
            second_battery = battery
        }
    }
    first_battery * 10 + second_battery
}

fn max_joltage_n<const N: usize>(batteries: &Vec<usize>) -> usize {
    // Start from the left and check for every battery if it is better than its previous
    let mut top_batteries = Vec::with_capacity(N);
    top_batteries.insert(0, batteries[0]);
    for battery_no in 1..batteries.len() {
        let battery = batteries[battery_no];
        let batteries_left = batteries.len() - battery_no;
        // Remove batteries if we have no more space for them
        while battery > top_batteries[top_batteries.len().saturating_sub(1)]
            && batteries_left > N - top_batteries.len()
        {
            top_batteries.pop();
            if top_batteries.len() == 0 {
                break;
            }
        }
        if top_batteries.len() < N {
            top_batteries.push(battery);
        }
    }
    let mut res = 0;
    for (dec, v) in top_batteries.iter().rev().enumerate() {
        res += 10_usize.pow(dec as u32) * v
    }
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let battery_banks = parse_digits(input);
    Some(
        battery_banks
            .iter()
            .map(|bank| {
                let res = max_joltage(bank);
                res as u64
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let battery_banks = parse_digits(input);
    Some(
        battery_banks
            .iter()
            .map(|bank| {
                let res = max_joltage_n::<12>(bank);
                res as u64
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
