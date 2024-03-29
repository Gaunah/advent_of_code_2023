use regex::Regex;

const PLACEHOLDER: char = '.';

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    parse_input(input, sum_part_numbers)
}

fn part2(input: &str) -> u32 {
    parse_input(input, sum_gear_ratios)
}

fn parse_input<F>(input: &str, process_window_fn: F) -> u32
where
    F: Fn([&str; 3], usize) -> u32,
{
    let line_len = input.lines().next().expect("Should not be empty!").len();
    let dots = PLACEHOLDER.to_string().repeat(line_len);
    std::iter::once(dots.as_str())
        .chain(input.lines())
        .chain(std::iter::once(dots.as_str()))
        .collect::<Vec<&str>>()
        .windows(3)
        .fold(0, |total, window| {
            if let [above, mid, below] = window {
                total + process_window_fn([above, mid, below], line_len)
            } else {
                total
            }
        })
}

fn sum_part_numbers(window: [&str; 3], line_len: usize) -> u32 {
    let re = Regex::new(r"(\d+)").expect("Should be valid regex!");
    let [above, mid, below] = window;

    re.captures_iter(mid)
        .filter_map(|cap| {
            let number = cap.get(0).unwrap();
            let start = number.start().saturating_sub(1).clamp(0, line_len);
            let end = number.end().saturating_add(1).clamp(0, line_len);
            if contains_symbol(&above[start..end])
                || contains_symbol(&mid[start..end])
                || contains_symbol(&below[start..end])
            {
                Some(cap[0].parse::<u32>().expect("Should be a positive number!"))
            } else {
                None
            }
        })
        .sum()
}

fn contains_symbol(slice: &str) -> bool {
    slice
        .find(|c: char| !c.is_ascii_digit() && c != PLACEHOLDER)
        .is_some()
}

fn sum_gear_ratios(window: [&str; 3], line_len: usize) -> u32 {
    let re = Regex::new(r"(\d+)").expect("Should be valid regex!");
    let [above, mid, below] = window;

    let gear_numbers: Vec<_> = re
        .captures_iter(above)
        .chain(re.captures_iter(mid))
        .chain(re.captures_iter(below))
        .map(|cap| cap.get(0).unwrap())
        .collect();

    mid.match_indices('*')
        .filter_map(|(i, _)| {
            let connected_numbers: Vec<u32> = gear_numbers
                .iter()
                .filter_map(|number| {
                    let start = number.start().saturating_sub(1);
                    let end = number.end().saturating_add(1);
                    let clamped_range = start..end.clamp(0, line_len);
                    if clamped_range.contains(&i) {
                        Some(
                            number
                                .as_str()
                                .parse()
                                .expect("Should be a positive number!"),
                        )
                    } else {
                        None
                    }
                })
                .collect();
            if connected_numbers.len() == 2 {
                Some(connected_numbers[0] * connected_numbers[1])
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 4361);
    }

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 467835);
    }
}
