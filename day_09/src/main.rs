fn main() {
    let input = include_str!("../input.txt");
    println!("Answer process_input: {}", process_input(input, false));
    println!("Answer process_input: {}", process_input(input, true));
}

fn process_input(input: &str, reverse: bool) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut data = line
                .split_whitespace()
                .filter_map(|parts| parts.parse().ok())
                .collect::<Vec<i32>>();

            if reverse {
                data.reverse();
            }

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

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn case1() {
        assert_eq!(process_input(TEST_INPUT, false), 114);
    }

    #[test]
    fn case2() {
        assert_eq!(process_input(TEST_INPUT, true), 2);
    }
}
