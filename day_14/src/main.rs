fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut grid = transpose(&parse_input(input));
    sort_rows(&mut grid);

    // count load per column
    let mut total_load = 0;
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            if grid[row][col] == 'O' {
                total_load += grid[0].len() - col;
            }
        }
    }

    total_load
}

fn sort_rows(grid: &mut [Vec<char>]) {
    grid.iter_mut().for_each(|row| {
        // just bubble sort
        for i in 0..row.len() {
            for j in 0..(row.len() - i - 1) {
                if (row[j], row[j + 1]) == ('.', 'O') {
                    row.swap(j, j + 1);
                }
            }
        }
    })
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

#[allow(dead_code)]
fn print_grid(grid: &[Vec<char>]) {
    for line in grid.iter() {
        for ch in line.iter() {
            print!("{ch}");
        }
        println!();
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// fn part2(input: &str) -> u32 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 136);
    }

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
