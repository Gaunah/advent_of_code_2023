use petgraph::graph::Graph;
use petgraph::visit::EdgeRef;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    println!("Answer part2: {}", part2(input));
}

fn parse_input(input: &str) -> (&str, Graph<String, char>) {
    let mut lines = input.lines();
    let instructions: &str = lines.next().expect("Should not be empty!");
    lines.next(); // skip the empty line

    let mut graph = Graph::new();
    let mut nodes = HashMap::new();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    for line in lines {
        let Some(caps) = re.captures(line) else {
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

fn part1(input: &str) -> u64 {
    let (instructions, graph) = parse_input(input);
    get_step_count(instructions, &graph, "AAA", |node| node == "ZZZ")
}

fn get_step_count(
    instructions: &str,
    graph: &Graph<String, char>,
    start: &str,
    end_condition: impl Fn(&str) -> bool,
) -> u64 {
    let mut current_node = graph
        .node_indices()
        .find(|&node| graph[node] == start)
        .expect("Graph should contain start node!");

    let mut step_count = 0;
    for direction in instructions.chars().cycle() {
        if let Some(edge) = graph
            .edges(current_node)
            .find(|edge| edge.weight() == &direction)
        {
            current_node = edge.target();
        } else {
            panic!("No valid edge found for direction {}", direction);
        }

        step_count += 1;
        if end_condition(&graph[current_node]) {
            break;
        }
    }

    step_count
}

fn part2(input: &str) -> u64 {
    let (instructions, graph) = parse_input(input);

    let all_start_nodes = graph
        .node_indices()
        .filter(|&node| graph[node].ends_with('A'));

    let step_count_vec: Vec<u64> = all_start_nodes
        .map(|node| {
            get_step_count(instructions, &graph, &graph[node], |node| {
                node.ends_with('Z')
            })
        })
        .collect();

    lcm_of_vec(&step_count_vec)
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn lcm_of_vec(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .fold(1, |acc, &num| lcm(acc, std::cmp::max(num, 1)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let input1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input1), 2);

        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input2), 6);
    }

    #[test]
    fn case2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX) ";
        assert_eq!(part2(input), 6);
    }
}
