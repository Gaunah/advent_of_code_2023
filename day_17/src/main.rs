use std::collections::BinaryHeap;

use colored::Colorize;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).expect("Should only contain digits") as u8)
                .collect()
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    heat_loss: usize,
    pos: (usize, usize),
    trail: Vec<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.heat_loss.cmp(&self.heat_loss))
    }
}

fn manhattan_dist(p1: (usize, usize), p2: (usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn best_path(grid: &[Vec<u8>], start: (usize, usize), goal: (usize, usize)) -> Option<State> {
    let mut seen: Vec<Vec<usize>> = grid
        .iter()
        .map(|row| row.iter().map(|_| usize::MAX).collect())
        .collect();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    seen[start.0][start.1] = 0;
    queue.push(State {
        heat_loss: 0,
        pos: start,
        trail: vec![start],
    });

    let mut all_states = vec![];
    while let Some(State {
        heat_loss,
        pos,
        trail,
    }) = queue.pop()
    {
        if pos == goal {
            all_states.push(State {
                heat_loss,
                pos,
                trail: trail.clone(),
            });
        }

        if heat_loss > seen[pos.0][pos.1] {
            continue;
        }

        for coord in can_visit_next(grid, pos, &trail) {
            let new_heat_loss = heat_loss + grid[coord.0][coord.1] as usize;
            if new_heat_loss < seen[coord.0][coord.1] {
                let mut new_trail = trail.clone();
                new_trail.push(coord);
                let next = State {
                    heat_loss: new_heat_loss,
                    pos: coord,
                    trail: new_trail,
                };
                queue.push(next);
                seen[coord.0][coord.1] = new_heat_loss;
            }
        }
    }

    dbg!(all_states.len());
    all_states.into_iter().min()
}

fn can_visit_next(
    grid: &[Vec<u8>],
    current_pos: (usize, usize),
    previous: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let mut out = vec![];
    let current_pos = (current_pos.0 as i32, current_pos.1 as i32);
    for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        if let Some(straight_dir) = three_in_row(previous) {
            if straight_dir == dir {
                continue;
            }
        }
        let new_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);

        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 as usize >= grid.len()
            || new_pos.1 as usize >= grid[0].len()
            || previous.contains(&(new_pos.0 as usize, new_pos.1 as usize))
        {
            continue;
        }

        out.push((new_pos.0 as usize, new_pos.1 as usize));
    }

    out
}

fn three_in_row(coords: &[(usize, usize)]) -> Option<(i32, i32)> {
    if coords.len() < 3 {
        return None;
    }

    let coords = &coords[coords.len() - 3..];

    if coords[0].0 == coords[1].0 && coords[1].0 == coords[2].0 {
        // row stays the same
        return Some(if coords[0].1 > coords[2].1 {
            (0, -1)
        } else {
            (0, 1)
        });
    } else if coords[0].1 == coords[1].1 && coords[1].1 == coords[2].1 {
        // col stays the same
        return Some(if coords[0].0 > coords[2].0 {
            (-1, 0)
        } else {
            (1, 0)
        });
    }

    None
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let start = (0, 0);
    let goal = (grid.len() - 1, grid[0].len() - 1);

    let out = best_path(&grid, start, goal);

    for (row, line) in grid.iter().enumerate() {
        for (col, num) in line.iter().enumerate() {
            if out.as_ref().unwrap().trail.contains(&(row, col)) {
                print!("{}", format!("{num}").green());
            } else {
                print!("{num}");
            }
        }
        println!();
    }

    out.unwrap().heat_loss
}

// fn part2(input: &str) -> u32 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 102);
    }

    #[test]
    fn case2() {
        let input = "11199
12199
99199
99131
99111";
        assert_eq!(part1(input), 9);
    }

    #[test]
    fn test_three_in_row() {
        let points = [(1, 2), (1, 3), (1, 4)];
        let points2 = [(2, 3), (3, 3), (4, 3)];
        let points3 = [(1, 1), (2, 2), (3, 3)];
        let points4 = [(1, 3), (1, 2), (1, 1)];
        let points5 = [(6, 3), (5, 3), (4, 3)];
        assert_eq!(three_in_row(&points), Some((0, 1)));
        assert_eq!(three_in_row(&points2), Some((1, 0)));
        assert_eq!(three_in_row(&points3), None);
        assert_eq!(three_in_row(&points4), Some((0, -1)));
        assert_eq!(three_in_row(&points5), Some((-1, 0)));
    }

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
