use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    #[inline]
    fn reflect_slash(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }

    #[inline]
    fn reflect_back_slash(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> usize {
    get_energized_tiles(&parse_input(input), (0, 0), Direction::Right).len()
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    assert!(grid.len() == grid[0].len());
    let grid_size = grid.len();

    (0..grid_size)
        .into_par_iter()
        .flat_map(|i| {
            vec![
                // top row
                get_energized_tiles(&grid, (0, i), Direction::Down).len(),
                // bottom row
                get_energized_tiles(&grid, (grid_size - 1, i), Direction::Up).len(),
                // left col
                get_energized_tiles(&grid, (i, 0), Direction::Right).len(),
                // right col
                get_energized_tiles(&grid, (i, grid_size - 1), Direction::Left).len(),
            ]
        })
        .max()
        .expect("Should not be empty!")
}

fn get_energized_tiles(
    grid: &[Vec<char>],
    start_coord: (usize, usize),
    start_dir: Direction,
) -> HashMap<(usize, usize), u8> {
    let mut visited: HashMap<(usize, usize), u8> = HashMap::new();
    walk(grid, start_coord, start_dir, &mut visited);
    visited
}

fn walk(
    grid: &[Vec<char>],
    start_coord: (usize, usize),
    start_dir: Direction,
    visited: &mut HashMap<(usize, usize), u8>,
) {
    let mut current_coord = start_coord;
    let mut current_dir = start_dir;

    while current_coord.0 < grid.len() && current_coord.1 < grid[0].len() {
        if visited.get(&current_coord).unwrap_or(&0) > &4 {
            // if one tile gets more than 4 times engergized we hit a cycel
            break;
        }

        visited
            .entry(current_coord)
            .and_modify(|val| *val += 1)
            .or_insert(1);

        match (&current_dir, grid[current_coord.0][current_coord.1]) {
            (Direction::Left, '|') | (Direction::Right, '|') => {
                walk(grid, current_coord, Direction::Up, visited);
                walk(grid, current_coord, Direction::Down, visited);
                break;
            }
            (Direction::Up, '-') | (Direction::Down, '-') => {
                walk(grid, current_coord, Direction::Left, visited);
                walk(grid, current_coord, Direction::Right, visited);
                break;
            }
            (dir, '/') => current_dir = dir.reflect_slash(),
            (dir, '\\') => current_dir = dir.reflect_back_slash(),
            _ => {} // just keep the current_dir
        };

        current_coord = update_coord(current_coord, &current_dir);
    }
}

#[inline]
fn update_coord((row, col): (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (row.wrapping_sub(1), col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col.wrapping_sub(1)),
        Direction::Right => (row, col + 1),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 46);
    }

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 51);
    }
}
