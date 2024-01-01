use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> usize {
    get_energized_tiles(&parse_input(input)).len()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn reflect_slash(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }

    fn reflect_back_slash(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }
}

fn get_energized_tiles(grid: &[Vec<char>]) -> HashMap<(usize, usize), u8> {
    let mut visited: HashMap<(usize, usize), u8> = HashMap::new();
    walk(grid, (0, 0), Direction::Right, &mut visited);

    // for (row, line) in grid.iter().enumerate() {
    //     for (col, ch) in line.iter().enumerate() {
    //         if visited.contains_key(&(row, col)) {
    //             print!("#");
    //         } else {
    //             print!("{ch}");
    //         }
    //     }
    //     println!();
    // }
    visited
}

fn walk(
    grid: &[Vec<char>],
    current_coord: (usize, usize),
    current_dir: Direction,
    visited: &mut HashMap<(usize, usize), u8>,
) {
    let mut current_coord = current_coord;
    let mut current_dir = current_dir;

    while current_coord.0 < grid.len() && current_coord.1 < grid[0].len() {
        if visited.get(&current_coord).unwrap_or(&0) > &4 {
            // break cycels
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
            (dir, _) => current_dir = *dir,
        };

        current_coord = update_coord(current_coord, &current_dir);
    }
}

fn update_coord((row, col): (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (row.wrapping_sub(1), col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col.wrapping_sub(1)),
        Direction::Right => (row, col + 1),
    }
}

// fn part2(input: &str) -> u32 {
//     0
// }

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

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
