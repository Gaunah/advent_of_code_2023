fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn part1(input: &str) -> u32 {
    parse_input_p1(input)
        .iter()
        .fold(1, |acc, race| acc * nums_of_ways_to_win(race))
}

fn parse_input_p1(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect();

    let distances: Vec<u64> = lines
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_input_p2(input: &str) -> Race {
    let lines: Vec<u64> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(0)
        })
        .collect();

    Race {
        time: lines[0],
        distance: lines[1],
    }
}

fn part2(input: &str) -> u32 {
    nums_of_ways_to_win(&parse_input_p2(input))
}

fn nums_of_ways_to_win(race: &Race) -> u32 {
    // Formula: dist = x*(time-x)
    // dist + 1 to win
    // Calc roots of the quadratic equation
    // Then calc ints in the range between roots

    let time = race.time as f64;
    let distance = race.distance as f64 + 1.0; // +1 to win
    let discriminant = time.powi(2) - 4.0 * distance;

    if discriminant >= 0.0 {
        // Real roots exist
        let root1 = (-time + discriminant.sqrt()) / -2.0;
        let root2 = (-time - discriminant.sqrt()) / -2.0;
        (root2.floor() - root1.ceil()) as u32 + 1
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 288);
    }

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 71503);
    }
}
