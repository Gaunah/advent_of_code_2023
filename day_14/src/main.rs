use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut cache = HashMap::new();
    let mut cycle_start = 0;
    let mut cycle_length = 0;

    for i in 0..1000000000 {
        sort_columns(&mut grid, true);
        sort_rows(&mut grid, true);
        sort_columns(&mut grid, false);
        sort_rows(&mut grid, false);
        if let Some(&first_seen) = cache.get(&grid) {
            // Cycle detected
            cycle_start = first_seen;
            cycle_length = i - first_seen;
            break;
        } else {
            // should rather hash the grid instead of cloning the whole grid, but whatever^^
            cache.insert(grid.clone(), i);
        }
    }

    let equivalent_iteration = (1000000000 - cycle_start) % cycle_length + cycle_start - 1;
    let final_grid = cache
        .iter()
        .find(|&(_, &index)| index == equivalent_iteration)
        .unwrap()
        .0;

    // Return the desired metric from the final grid
    get_total_load_north(final_grid)
}

fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);
    sort_columns(&mut grid, true);
    get_total_load_north(&grid)
}

fn get_total_load_north(grid: &[Vec<char>]) -> usize {
    let grid_rows = grid.len();
    grid.iter()
        .enumerate()
        .map(|(num, line)| line.iter().filter(|&&ch| ch == 'O').count() * (grid_rows - num))
        .sum()
}

fn sort_rows(grid: &mut [Vec<char>], to_west: bool) {
    let pattern = match to_west {
        true => ('.', 'O'),
        false => ('O', '.'),
    };

    grid.par_iter_mut().for_each(|row| {
        // just bubble sort
        for i in 0..row.len() {
            for j in 0..(row.len() - i - 1) {
                if (row[j], row[j + 1]) == pattern {
                    row.swap(j, j + 1);
                }
            }
        }
    })
}

fn sort_columns(grid: &mut [Vec<char>], to_north: bool) {
    let pattern = match to_north {
        true => ('.', 'O'),
        false => ('O', '.'),
    };

    let num_columns = grid[0].len();
    let num_rows = grid.len();

    for col in 0..num_columns {
        for i in 0..num_rows {
            for j in 0..num_rows - i - 1 {
                if (grid[j][col], grid[j + 1][col]) == pattern {
                    let temp = std::mem::replace(&mut grid[j][col], 'X');
                    grid[j][col] = std::mem::replace(&mut grid[j + 1][col], temp);
                }
            }
        }
    }
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

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 64);
    }
}
