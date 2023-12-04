use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn slice_to_hashset(slice: &str) -> HashSet<u32> {
    slice
        .split(' ')
        .filter_map(|part| part.parse::<u32>().ok())
        .collect()
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let (win_line, my_line) = line.split_once("|").unwrap();

            let number_of_matches = slice_to_hashset(win_line)
                .intersection(&slice_to_hashset(my_line))
                .count();

            if number_of_matches > 0 {
                2u32.pow((number_of_matches - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    0
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
