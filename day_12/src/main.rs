fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (spring, config) = line.split_once(' ').unwrap();
        let config: Vec<u8> = config.split(',').map(|num| num.parse().unwrap()).collect();
        sum += num_of_fitting_variations(spring, &config);
    }
    sum
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (spring_floded, config_folded) = line.split_once(' ').unwrap();
        let mut config_folded: Vec<u8> = config_folded
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();

        let mut spring: String = String::new();
        let mut config: Vec<u8> = vec![];
        for _ in 0..5 {
            spring += spring_floded;
            config.append(&mut config_folded);
        }

        sum += num_of_fitting_variations(&spring, &config);
    }

    sum
}

fn num_of_fitting_variations(unknown_spring: &str, config: &[u8]) -> usize {
    generate_variations(unknown_spring)
        .iter()
        .filter(|var| spring_fits_config(var, config))
        .count()
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
        assert!(spring_fits_config("#.#.###", &[1, 1, 3]));
        assert!(spring_fits_config(".#...#....###.", &[1, 1, 3]));
        assert!(spring_fits_config(".#.###.#.######", &[1, 3, 1, 6]));
        assert!(spring_fits_config("####.#...#...", &[4, 1, 1]));
        assert!(spring_fits_config("#....######..#####.", &[1, 6, 5]));
        assert!(spring_fits_config(".###.##....#", &[3, 2, 1]));

        assert!(!spring_fits_config("#.#.###", &[3, 1, 3]));
        assert!(!spring_fits_config(".#...#....###.", &[1, 3, 3]));
        assert!(!spring_fits_config(".#.###.#.######", &[1, 1, 1, 6]));
        assert!(!spring_fits_config("####.#...#...", &[1, 1, 1]));
        assert!(!spring_fits_config("#....######..#####.", &[1, 9, 5]));
        assert!(!spring_fits_config(".###.##....#", &[3, 2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 525152);
    }
}
