fn main() {
    let input = include_str!("../input.txt").replace("\r\n", "\n\n");
    println!("Answer part1: {}", part1(&input, 0));
    println!("Answer part2: {}", part1(&input, 1));
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|block| block.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn transpose(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut transposed = vec![vec![' '; matrix.len()]; matrix[0].len()];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }

    transposed
}

fn print_map(map: &[Vec<char>]) {
    for line in map.iter() {
        for ch in line.iter() {
            print!("{ch}");
        }
        println!();
    }
}

fn part1(input: &str, errors_needed: u32) -> usize {
    let out = parse_input(input);

    out.iter()
        .map(|block| {
            //find splitting row
            let row = find_mirror_pos(block, errors_needed);
            let mut col = None;
            if row.is_none() {
                col = find_mirror_pos(&transpose(block), errors_needed);
            }

            match (row, col) {
                (None, Some(val)) => val,
                (Some(val), None) => val * 100,
                _ => {
                    print_map(block);
                    unreachable!()
                }
            }
        })
        .sum()
}

fn find_mirror_pos(map: &[Vec<char>], errors_needed: u32) -> Option<usize> {
    let len = map.len();
    let mut out = None;

    for i in 1..len {
        let (first, second) = map.split_at(i);
        let mut second = second.to_owned();
        let mut first = first.to_owned();

        let min_len = std::cmp::min(first.len(), second.len());
        if second.len() > first.len() {
            second.truncate(min_len);
            second.reverse();
        } else {
            first.reverse();
            first.truncate(min_len);
        }

        if are_blocks_equal(&first, &second, errors_needed) {
            out = Some(i);
            break;
        }
    }

    out
}

fn are_blocks_equal(first: &[Vec<char>], second: &[Vec<char>], errors_needed: u32) -> bool {
    let mut errors = 0;
    for row in 0..first.len() {
        for col in 0..first[0].len() {
            if first[row][col] != second[row][col] {
                errors += 1;
            }
        }
    }

    errors_needed == errors
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT, 0), 405);
    }

    #[test]
    fn case2() {
        assert_eq!(part1(TEST_INPUT, 1), 400);
    }
}
