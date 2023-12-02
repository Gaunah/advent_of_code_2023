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
            if is_any_over_max(&re_red, max_red, line)
                || is_any_over_max(&re_green, max_green, line)
                || is_any_over_max(&re_blue, max_blue, line)
            {
                None // found too many cubes of one color in the line
            } else {
                Some(
                    re_game.captures(line).expect("Should contain game ID!")["game_id"]
                        .parse::<u32>()
                        .expect("Should be a positiv number!"),
                )
            }
        })
        .sum()
}

fn is_any_over_max(re: &Regex, max: u32, line: &str) -> bool {
    re.captures_iter(line)
        .any(|cap| cap[1].parse::<u32>().expect("Should be a positiv number!") > max)
}

fn part2(input: &str) -> u32 {
    let re_red = Regex::new(RE_RED).expect("Should be valid regex!");
    let re_green = Regex::new(RE_GREEN).expect("Should be valid regex!");
    let re_blue = Regex::new(RE_BLUE).expect("Should be valid regex!");

    input
        .lines()
        .map(|line| {
            let min_red_needed = extract_max(&re_red, line);
            let min_green_needed = extract_max(&re_green, line);
            let min_blue_needed = extract_max(&re_blue, line);

            min_red_needed * min_green_needed * min_blue_needed
        })
        .sum()
}

fn extract_max(re: &Regex, line: &str) -> u32 {
    re.captures_iter(line)
        .map(|cap| {
            cap[1]
                .parse::<u32>()
                .expect("Should always be a positiv number!")
        })
        .max()
        .expect("Should not be empty!")
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
