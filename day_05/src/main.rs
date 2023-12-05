fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let mut blocks = input.split("\n\n");
    // first block contains the seeds
    // every following block contains a map x => y
    let seeds = block_to_vec(blocks.next().unwrap());
    let maps: Vec<Vec<i64>> = blocks.map(|block| block_to_vec(block)).collect();

    seeds
        .iter()
        .map(|seed| {
            let mut out = *seed;

            for map in &maps {
                for chunk in map.chunks_exact(3) {
                    let [dst_start, src_start, range] = chunk else {
                        panic!("Chunk has less than 3 elements!");
                    };

                    if (src_start..&(src_start + range)).contains(&&out) {
                        // translate to next category
                        out += *dst_start - *src_start;
                        break;
                    }
                }
            }
            out
        })
        .min()
        .expect("Should not be empty!")
}

fn block_to_vec(block: &str) -> Vec<i64> {
    block
        .split_whitespace()
        .filter_map(|part| part.parse::<i64>().ok())
        .collect()
}

// fn part2(input: &str) -> u32 {
//     0
// }

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

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
