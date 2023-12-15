fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

#[derive(Debug, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> usize {
        other.x.abs_diff(self.x) + other.y.abs_diff(self.y)
    }
}

fn part1(input: &str) -> usize {
    let universe = read_and_expand_universe(input);
    let galaxies = find_all_galaxies(&universe);

    let mut dist_sum = 0;
    for i in 0..galaxies.len() {
        let mut rotated = galaxies.to_vec();
        rotated.rotate_left(i);
        for j in 1..galaxies.len() - i {
            dist_sum += rotated[0].distance(&rotated[j]);
        }
    }

    dist_sum
}

fn find_all_galaxies(universe: &[Vec<char>]) -> Vec<Coordinate> {
    let mut coords = vec![];
    for (y, row) in universe.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val == '#' {
                coords.push(Coordinate { x, y });
            }
        }
    }

    coords
}

fn read_and_expand_universe(input: &str) -> Vec<Vec<char>> {
    let mut vertical_expaned: Vec<Vec<char>> = vec![];
    vertical_expaned.reserve(140);

    for line in input.lines() {
        vertical_expaned.push(line.chars().collect());
        if line.chars().all(|ch| ch == '.') {
            vertical_expaned.push(line.chars().collect());
        }
    }

    let transposed = transpose(&vertical_expaned);

    let mut universe_t: Vec<Vec<char>> = vec![];
    universe_t.reserve(transposed.len());

    for line in transposed.iter() {
        universe_t.push(line.to_vec());
        if line.iter().all(|&ch| ch == '.') {
            universe_t.push(line.to_vec());
        }
    }

    transpose(&universe_t)
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

// fn part2(input: &str) -> u32 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 374);
    }

    #[test]
    fn case1_2() {
        let output = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
        let mut string = String::new();
        for line in read_and_expand_universe(TEST_INPUT).iter() {
            let row: String = line.iter().collect();
            string.push_str(&row);
            string.push('\n');
        }
        assert_eq!(string.trim_end(), output);
    }
    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
