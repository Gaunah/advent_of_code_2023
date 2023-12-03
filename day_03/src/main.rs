use regex::Regex;
use std::cmp;

const PLACEHOLDER: char = '.';

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn contains_symbol(slice: &str) -> bool {
    slice
        .find(|c: char| !c.is_digit(10) && c != PLACEHOLDER)
        .is_some()
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"(\d+)").expect("Should be valid regex!");

    let line_len = input.lines().next().expect("Should not be empty!").len();
    let dots = PLACEHOLDER.to_string().repeat(line_len);
    std::iter::once(dots.as_str())
        .chain(input.lines())
        .chain(std::iter::once(dots.as_str()))
        .collect::<Vec<&str>>()
        .windows(3)
        .fold(0, |total, window| {
            if let [above, mid, below] = window {
                total
                    + re.captures_iter(mid)
                        .filter_map(|cap| {
                            let c = cap.get(0).unwrap();
                            let start_e = c.start().saturating_sub(1);
                            let end_e = cmp::min(c.end() + 1, line_len);
                            if contains_symbol(&above[start_e..end_e])
                                || contains_symbol(&mid[start_e..end_e])
                                || contains_symbol(&below[start_e..end_e])
                            {
                                Some(cap[0].parse::<u32>().expect("Should be a positiv number!"))
                            } else {
                                None
                            }
                        })
                        .sum::<u32>()
            } else {
                total
            }
        })
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(\d+)").expect("Should be valid regex!");

    let line_len = input.lines().next().expect("Should not be empty!").len();
    let dots = PLACEHOLDER.to_string().repeat(line_len);
    std::iter::once(dots.as_str())
        .chain(input.lines())
        .chain(std::iter::once(dots.as_str()))
        .collect::<Vec<&str>>()
        .windows(3)
        .fold(0, |total, window| {
            if let [a, m, b] = window {
                total
                    + m.match_indices("*")
                        .filter_map(|(i, _)| {
                            let gear_numbers: Vec<u32> = re
                                .captures_iter(a)
                                .chain(re.captures_iter(m))
                                .chain(re.captures_iter(b))
                                .map(|cap| cap.get(0).unwrap())
                                .filter_map(|number| {
                                    let start_e = number.start().saturating_sub(1);
                                    let end_e = cmp::min(number.end() + 1, line_len);
                                    if (start_e..end_e).contains(&i) {
                                        Some(
                                            number
                                                .as_str()
                                                .parse()
                                                .expect("Should be a positiv number!"),
                                        )
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            if gear_numbers.len() == 2 {
                                Some(gear_numbers.first().unwrap() * gear_numbers.last().unwrap())
                            } else {
                                None
                            }
                        })
                        .sum::<u32>()
            } else {
                total
            }
        })
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
