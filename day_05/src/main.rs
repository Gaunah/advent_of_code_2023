use rayon::prelude::*;

fn main() {
    let input = include_str!("../input.txt").replace("\r\n", "\n");

    println!("Answer part1: {}", part1(&input));
    println!("Answer part2: {}", part2(&input));
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<i64>>) {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let maps: Vec<Vec<i64>> = blocks[1..]
        .iter()
        .map(|&block| block_to_vec(&block))
        .collect();
    let seed_line = block_to_vec(&blocks[0]);

    (seed_line, maps)
}

fn part1(input: &str) -> i64 {
    let (seed_line, maps) = parse_input(input);

    seed_line
        .par_iter()
        .map(|seed| get_location(*seed, &maps))
        .min()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let (seed_line, maps) = parse_input(input);

    seed_line
        .par_chunks_exact(2)
        .flat_map_iter(|chunk| match chunk {
            [start, range] => *start..(*start + range),
            _ => unreachable!(),
        })
        .map(|seed| get_location(seed, &maps))
        .min()
        .unwrap()
}

fn block_to_vec(block: &&str) -> Vec<i64> {
    block
        .split_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect()
}

fn get_location(seed: i64, maps: &[Vec<i64>]) -> i64 {
    let mut current = seed;

    for map in maps {
        for chunk in map.chunks_exact(3) {
            let [dst_start, src_start, range] = chunk else {
                continue; // should never happen
            };

            if (src_start..&(src_start + range)).contains(&&current) {
                // translate to next category
                current += *dst_start - *src_start;
                break;
            }
        }
    }
    current
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT), 35);
    }

    #[test]
    fn case2() {
        assert_eq!(part2(TEST_INPUT), 46);
    }
}
