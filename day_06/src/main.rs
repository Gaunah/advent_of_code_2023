fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn parse_input(input: &str) -> Vec<Race> {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|part| part.parse::<u32>().ok())
                .collect()
        })
        .collect();

    lines[0]
        .iter()
        .zip(&lines[1])
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    parse_input(input)
        .iter()
        .fold(1, |acc, race| acc * nums_of_ways_to_win(race))
}

fn nums_of_ways_to_win(race: &Race) -> u32 {
    // formula: dist = x*(time-x)
    // dist + 1 to win
    // calculate roots of the parabola
    // 0 = x*(time-x)-dist+1
    // then calc ints in the range between roots

    let time = race.time as f64;
    let distance = 1.0 + race.distance as f64; // +1 to win
    let discriminant = time.powi(2) - 4.0 * distance;

    if discriminant >= 0.0 {
        // Real roots exist
        let root1 = (-time + discriminant.sqrt()) / -2.0;
        let root2 = (-time - discriminant.sqrt()) / -2.0;
        let ints_in_range = (root2.floor() - root1.ceil()) as u32 + 1;

        ints_in_range
    } else {
        0
    }
}

// fn part2(input: &str) -> u32 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 288);
    }

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
