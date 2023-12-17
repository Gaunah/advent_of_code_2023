fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (spring, config) = line.split_once(' ').unwrap();
        let config: Vec<u8> = config.split(',').map(|num| num.parse().unwrap()).collect();
        let num_of_fitting_variations = generate_variations(spring)
            .iter()
            .filter(|var| spring_fits_config(var, &config))
            .count();
        acc + num_of_fitting_variations
    })
}

fn spring_fits_config(spring: &str, config: &[u8]) -> bool {
    let parts = spring.split('.').filter(|&x| !x.is_empty());

    if parts.clone().count() != config.len() {
        return false;
    }

    parts
        .zip(config)
        .all(|(part, &num)| part.len() == num as usize)
}

fn generate_variations(base: &str) -> Vec<String> {
    fn recurse(s: &str, current: &str, variations: &mut Vec<String>) {
        match s.find('?') {
            Some(index) => {
                let (left, right) = s.split_at(index);
                let right = &right[1..]; // Skip the '?'

                recurse(right, &format!("{}{}.", current, left), variations);
                recurse(right, &format!("{}{}#", current, left), variations);
            }
            None => variations.push(format!("{}{}", current, s)),
        }
    }

    let mut variations = Vec::new();
    recurse(base, "", &mut variations);
    variations
}

// fn part2(input: &str) -> u32 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn spring_fits() {
        assert!(spring_fits_config("#.#.###", &vec![1, 1, 3]));
        assert!(spring_fits_config(".#...#....###.", &vec![1, 1, 3]));
        assert!(spring_fits_config(".#.###.#.######", &vec![1, 3, 1, 6]));
        assert!(spring_fits_config("####.#...#...", &vec![4, 1, 1]));
        assert!(spring_fits_config("#....######..#####.", &vec![1, 6, 5]));
        assert!(spring_fits_config(".###.##....#", &vec![3, 2, 1]));

        assert!(!spring_fits_config("#.#.###", &vec![3, 1, 3]));
        assert!(!spring_fits_config(".#...#....###.", &vec![1, 3, 3]));
        assert!(!spring_fits_config(".#.###.#.######", &vec![1, 1, 1, 6]));
        assert!(!spring_fits_config("####.#...#...", &vec![1, 1, 1]));
        assert!(!spring_fits_config("#....######..#####.", &vec![1, 9, 5]));
        assert!(!spring_fits_config(".###.##....#", &vec![3, 2, 4]));
    }

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
