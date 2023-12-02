use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
}

fn part1(input: &str) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let re_red = Regex::new(r"(?<red>\d+) red").expect("Should be valid regex!");
    let re_green = Regex::new(r"(?<green>\d+) green").expect("Should be valid regex!");
    let re_blue = Regex::new(r"(?<blue>\d+) blue").expect("Should be valid regex!");
    let re_game = Regex::new(r"^Game (?<game_id>\d+)").expect("Should be valid regex!");

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

#[test]
fn case1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part1(input), 8);
}
