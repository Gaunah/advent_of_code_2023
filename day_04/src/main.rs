use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn slice_to_hashset(slice: &str) -> HashSet<u32> {
    slice
        .split_whitespace()
        .filter_map(|part| part.parse::<u32>().ok())
        .collect()
}

fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let (_, line) = line.split_once(": ").unwrap();
    let (win_line, my_line) = line.split_once("|").unwrap();
    (slice_to_hashset(win_line), slice_to_hashset(my_line))
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (win_set, my_set) = parse_line(line);
            let number_of_matches = win_set.intersection(&my_set).count();
            if number_of_matches > 0 {
                2u32.pow((number_of_matches - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut stash: Vec<u32> = vec![1; input.lines().count()];

    for (index, line) in input.lines().enumerate() {
        let (win_set, my_set) = parse_line(line);
        let number_of_matches = win_set.intersection(&my_set).count();

        let start = index + 1;
        let end = std::cmp::min(start + number_of_matches, stash.len());

        for i in start..end {
            stash[i] += stash[index]
        }
    }

    stash.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 30);
    }
}
