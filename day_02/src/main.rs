use regex::Regex;

const RE_RED: &str = r"(?<red>\d+) red";
const RE_GREEN: &str = r"(?<green>\d+) green";
const RE_BLUE: &str = r"(?<blue>\d+) blue";
const RE_GAME: &str = r"^Game (?<game_id>\d+)";

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let re_red = Regex::new(RE_RED).expect("Should be valid regex!");
    let re_green = Regex::new(RE_GREEN).expect("Should be valid regex!");
    let re_blue = Regex::new(RE_BLUE).expect("Should be valid regex!");
    let re_game = Regex::new(RE_GAME).expect("Should be valid regex!");

    input
        .lines()
        .filter_map(|line| {
            if let Some(_) = re_red
                .captures_iter(line)
                .find(|red_cap| red_cap["red"].parse::<u32>().unwrap() > max_red)
            {
                None // found too many red cubes in one line
            } else if let Some(_) = re_blue
                .captures_iter(line)
                .find(|blue_cap| blue_cap["blue"].parse::<u32>().unwrap() > max_blue)
            {
                None // found too many blue cubes in one line
            } else if let Some(_) = re_green
                .captures_iter(line)
                .find(|green_cap| green_cap["green"].parse::<u32>().unwrap() > max_green)
            {
                None // found too many green cubes in one line
            } else {
                Some(
                    re_game
                        .captures(line)
                        .expect("Should always contain game ID!")["game_id"]
                        .parse::<u32>()
                        .expect("Should always be a positiv number!"),
                )
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let re_red = Regex::new(RE_RED).expect("Should be valid regex!");
    let re_green = Regex::new(RE_GREEN).expect("Should be valid regex!");
    let re_blue = Regex::new(RE_BLUE).expect("Should be valid regex!");

    input
        .lines()
        .map(|line| {
            let min_red_needed = re_red
                .captures_iter(line)
                .map(|cap| {
                    cap["red"]
                        .parse::<u32>()
                        .expect("Should always be a positiv number!")
                })
                .max()
                .expect("Should not be empty!");

            let min_green_needed = re_green
                .captures_iter(line)
                .map(|cap| {
                    cap["green"]
                        .parse::<u32>()
                        .expect("Should always be a positiv number!")
                })
                .max()
                .expect("Should not be empty!");

            let min_blue_needed = re_blue
                .captures_iter(line)
                .map(|cap| {
                    cap["blue"]
                        .parse::<u32>()
                        .expect("Should always be a positiv number!")
                })
                .max()
                .expect("Should not be empty!");

            min_red_needed * min_green_needed * min_blue_needed
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 8);
    }

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 2286);
    }
}
