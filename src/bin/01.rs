advent_of_code::solution!(1);

// note for self: read the f'ing prompt and examples thoroughly before starting :)
//
pub fn part_one(input: &str) -> Option<u32> {
    let res: u32 = input.split("\n").fold(0, |acc, cal| {
        // note for self: digits is `mut`-able because its an iterator that updates itself as we
        // progress
        let mut digits = cal.chars().filter_map(|s| s.to_digit(10));
        let t1 = digits.next().unwrap_or(0);
        let t2 = digits.last().unwrap_or(t1);
        acc + (t1 * 10 + t2)
    });
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
