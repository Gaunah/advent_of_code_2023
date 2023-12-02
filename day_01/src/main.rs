fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.lines().map(concat_first_and_last_digit).sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let processed_line = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");
            concat_first_and_last_digit(&processed_line)
        })
        .sum()
}

fn concat_first_and_last_digit(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
        first * 10 + last
    } else {
        0 // should never happen
    }
}

#[test]
fn case1() {
    let intput = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    assert_eq!(part1(intput), 142);
}

#[test]
fn case2() {
    let intput = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    assert_eq!(part2(intput), 281);
}
