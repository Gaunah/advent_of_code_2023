//use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::visit::EdgeRef;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
}

fn parse_input(input: &str) -> (&str, Graph<String, char>) {
    let mut lines = input.lines();
    let instructions: &str = lines.next().expect("Should not be empty!");
    lines.next(); // skip the empty line

    let mut graph = Graph::new();
    let mut nodes = HashMap::new();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    for line in lines {
        let Some(caps) = re.captures(&line) else {
            panic!("Invalid line format!");
        };

        let node_name = caps.get(1).unwrap().as_str().to_string();
        let con_left = caps.get(2).unwrap().as_str().to_string();
        let con_right = caps.get(3).unwrap().as_str().to_string();

        let node_index = *nodes
            .entry(node_name.clone())
            .or_insert_with(|| graph.add_node(node_name));

        let con_left_index = *nodes
            .entry(con_left.clone())
            .or_insert_with(|| graph.add_node(con_left));

        graph.add_edge(node_index, con_left_index, 'L');

        let con_right_index = *nodes
            .entry(con_right.clone())
            .or_insert_with(|| graph.add_node(con_right));
        graph.add_edge(node_index, con_right_index, 'R');
    }

    (instructions, graph)
}

fn part1(input: &str) -> u32 {
    let (instructions, graph) = parse_input(input);

    let mut current_node = graph
        .node_indices()
        .find(|&node| graph[node] == "AAA")
        .expect("Graph should contain node AAA!");

    let mut step_count = 0;
    for direction in instructions.chars().cycle() {
        current_node = graph
            .edges(current_node)
            .find_map(|edge| {
                if edge.weight() == &direction {
                    Some(edge.target())
                } else {
                    None
                }
            })
            .unwrap();

        step_count += 1;
        if graph[current_node] == String::from("ZZZ") {
            break;
        }
    }

    step_count
}

// fn part2(input: &str) -> u32 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn case1() {
        assert_eq!(part1(TEST_INPUT_1), 2);
        assert_eq!(part1(TEST_INPUT_2), 6);
    }

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
