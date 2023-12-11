fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let data = line
                .split_whitespace()
                .filter_map(|parts| parts.parse().ok())
                .collect::<Vec<i32>>();
            data.last().expect("Should not be empty!") + predict_next_val(&data)
        })
        .sum()
}

fn predict_next_val(data: &[i32]) -> i32 {
    if data.iter().all(|&num| num == 0) {
        return 0;
    }

    let diffs: Vec<i32> = data
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    diffs.last().expect("Should not be empty!") + predict_next_val(&diffs)
}

// fn part2(input: &str) -> u32 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 114);
    }

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
