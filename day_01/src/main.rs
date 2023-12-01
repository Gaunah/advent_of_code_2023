fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|d| d.to_digit(10).expect("should only be numbers"))
                .collect()
        })
        .fold(0, |sum, vec: Vec<u32>| {
            sum + vec.first().expect("should not be empty!") * 10
                + vec.last().expect("should not be empty!")
        })
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|d| d.to_digit(10).expect("should only be numbers"))
                .collect()
        })
        .fold(0, |sum, vec: Vec<u32>| {
            sum + vec.first().expect("should not be empty!") * 10
                + vec.last().expect("should not be empty!")
        })
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
