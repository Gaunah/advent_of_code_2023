fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

fn hash(string: &str) -> u8 {
    string
        .chars()
        .fold(0, |acc, ch| acc.wrapping_add(ch as u8).wrapping_mul(17))
}

fn part1(input: &str) -> u32 {
    input.trim().split(',').map(|step| hash(step) as u32).sum()
}

// fn part2(input: &str) -> u32 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_test() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 1320);
    }

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
