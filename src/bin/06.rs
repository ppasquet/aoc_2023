advent_of_code::solution!(6);
use regex::Regex;

fn read_numbers(row: &str) -> Vec<u64> {
    let numbers_regex = Regex::new(r"\d+").unwrap();
    return numbers_regex
        .find_iter(row)
        .filter_map(|m| m.as_str().parse::<u64>().ok()).collect::<Vec<_>>();
}

fn read_number(row: &str) -> u64 {
    let numbers_regex = Regex::new(r"\d+").unwrap();
    let number_str: String = numbers_regex.find_iter(row)
        .map(|m| m.as_str()).collect::<Vec<_>>().join("");
    number_str.as_str().parse::<u64>().ok().unwrap()
}

fn solve_ineq(time: u64, record: u64) -> std::ops::Range<u32> {
    let a = 1.0;
    let c = record as f64;
    let b = -(time as f64);
    let x1 = (-b - f64::sqrt(b.powi(2) - (4.0 * a * c))) / (2.0 * a);
    let x2 = (-b + f64::sqrt(b.powi(2) - (4.0 * a * c))) / (2.0 * a);
    let lb = if x1.trunc() == x1 { (x1 + 1.0) as u32 } else { x1.ceil() as u32 };
    let ub = x2.ceil() as u32;
    lb..ub
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut values = input.split_terminator("\n").map(read_numbers);
    let time = values.next().unwrap();
    let record = values.next().unwrap();
    let res = std::iter::zip(time, record).map(|(t, r)| {
        solve_ineq(t, r).len()
    });
    u32::try_from(res.product::<usize>()).ok()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut values = input.split_terminator("\n").map(read_number);
    let time = values.next().unwrap();
    let record = values.next().unwrap();
    let res = solve_ineq(time, record).len();
    u64::try_from(res).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
