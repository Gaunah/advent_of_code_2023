fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_coord = find_start_coord(&matrix).expect("Should contain start!");
    dbg!(start_coord);
    0
}

fn find_start_coord(matrix: &[Vec<char>]) -> Option<(usize, usize)> {
    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == 'S') {
            return Some((i, j));
        }
    }
    None
}

// fn part2(input: &str) -> u32 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1_1() {
        let input: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn case1_2() {
        let input: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(part1(input), 8);
    }

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
