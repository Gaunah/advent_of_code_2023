fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn part1(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut current_coord = find_start_coord(&matrix).expect("Should contain start!");
    let mut next_direction = Some(Direction::East); // Assumed first step East is valid
    let mut steps = 0;

    // loops until we hit 'S'
    while let Some(dir) = next_direction {
        current_coord = update_coord(current_coord, dir);
        next_direction = get_next_direction(&matrix, current_coord, dir);
        steps += 1;
    }
    steps / 2
}

fn update_coord((row, col): (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        Direction::North => (row.saturating_sub(1), col),
        Direction::East => (row, col + 1),
        Direction::South => (row + 1, col),
        Direction::West => (row, col.saturating_sub(1)),
    }
}

fn find_start_coord(matrix: &[Vec<char>]) -> Option<(usize, usize)> {
    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == 'S') {
            return Some((i, j));
        }
    }
    None
}

fn get_next_direction(
    matrix: &[Vec<char>],
    current: (usize, usize),
    from: Direction,
) -> Option<Direction> {
    let (row, col) = current;
    match (matrix[row][col], from) {
        ('|', Direction::North) => Some(Direction::North),
        ('|', Direction::South) => Some(Direction::South),

        ('-', Direction::East) => Some(Direction::East),
        ('-', Direction::West) => Some(Direction::West),

        ('L', Direction::South) => Some(Direction::East),
        ('L', Direction::West) => Some(Direction::North),

        ('J', Direction::South) => Some(Direction::West),
        ('J', Direction::East) => Some(Direction::North),

        ('7', Direction::North) => Some(Direction::West),
        ('7', Direction::East) => Some(Direction::South),

        ('F', Direction::North) => Some(Direction::East),
        ('F', Direction::West) => Some(Direction::South),
        ('S', _) => None,
        other => panic!("Invalid combination! {:?}", other),
    }
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
