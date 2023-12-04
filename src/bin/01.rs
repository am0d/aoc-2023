advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let first_digits = lines
        .clone()
        .filter_map(|line| line.chars().find(|c| c.is_digit(10)));
    let last_digits = lines.filter_map(|line| line.chars().rev().find(|c| c.is_digit(10)));
    let first_sum = first_digits
        .map(|c| c.to_digit(10).unwrap_or(0) * 10)
        .sum::<u32>();
    let last_sum = last_digits
        .map(|c| c.to_digit(10).unwrap_or(0))
        .sum::<u32>();
    Some(first_sum + last_sum)
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
        assert_eq!(result,None);
    }
}
