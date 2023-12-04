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
    let lines = input.lines();

    Some(
        lines
            .map(|line| {
                vec![
                    // Search from the front of the line
                    (0..line.len())
                        .find_map(|i| get_digit_maybe(&line[i..]))
                        .unwrap_or(0),
                    // Search from the back of the line
                    (0..line.len() + 1)
                        .find_map(|i| get_digit_maybe(&line[(line.len() - i)..]))
                        .unwrap_or(0),
                ]
            })
            .map(|line| line.iter().fold(0, |acc, elem| acc * 10 + elem))
            .sum::<u32>(),
    )
}

fn get_digit_maybe(input: &str) -> Option<u32> {
    match true {
        true if input.starts_with("one") || input.starts_with("1") => Some(1),
        true if input.starts_with("two") || input.starts_with("2") => Some(2),
        true if input.starts_with("three") || input.starts_with("3") => Some(3),
        true if input.starts_with("four") || input.starts_with("4") => Some(4),
        true if input.starts_with("five") || input.starts_with("5") => Some(5),
        true if input.starts_with("six") || input.starts_with("6") => Some(6),
        true if input.starts_with("seven") || input.starts_with("7") => Some(7),
        true if input.starts_with("eight") || input.starts_with("8") => Some(8),
        true if input.starts_with("nine") || input.starts_with("9") => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_part_two_edge_cases() {
        assert_eq!(part_two("3lnksqznzh\n"), Some(33));
        assert_eq!(part_two("sevenine"), Some(79));
    }

    #[test]
    fn test_part_two_part_one_input() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }
}
