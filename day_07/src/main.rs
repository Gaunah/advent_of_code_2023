use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", get_solution(input, false));
    println!("Answer part2: {}", get_solution(input, true));
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let cards = parts.next().expect("Should have first part!").to_string();

            let bid = parts
                .next()
                .expect("Should have second part!")
                .parse()
                .expect("Should be a positiv number!");

            Hand { cards, bid }
        })
        .collect()
}

fn cmp_hands(a: &Hand, b: &Hand, joker: bool) -> Ordering {
    match get_hand_type(a, joker).cmp(&get_hand_type(b, joker)) {
        Ordering::Equal => a
            .cards
            .chars()
            .zip(b.cards.chars())
            .find_map(|(first_card, second_card)| {
                let first_val = get_card_value(&first_card, joker);
                let second_val = get_card_value(&second_card, joker);
                (first_val != second_val).then(|| first_val.cmp(&second_val))
            })
            .unwrap_or(Ordering::Equal),
        other => other,
    }
}

fn process_joker(card_counts: &mut HashMap<char, u8>) {
    let joker_count = card_counts.remove(&'J').unwrap_or(0);
    let (&modal_card, _) = card_counts
        .iter()
        .max_by_key(|&(_, &val)| val)
        .expect("Should not be empty!");

    *(card_counts.get_mut(&modal_card).unwrap()) += joker_count;
}

fn get_hand_type(hand: &Hand, joker: bool) -> u8 {
    let mut card_counts = HashMap::new();
    for ch in hand.cards.chars() {
        *card_counts.entry(ch).or_insert(0) += 1;
    }

    if joker && card_counts.len() > 1 {
        process_joker(&mut card_counts);
    }

    let mut values: Vec<_> = card_counts.values().collect();
    values.sort_unstable();
    match values.as_slice() {
        [1, 1, 1, 1, 1] => 0, // high card
        [1, 1, 1, 2] => 1,    // one pair
        [1, 2, 2] => 2,       // two pair
        [1, 1, 3] => 3,       // three of a kind
        [2, 3] => 4,          // full house
        [1, 4] => 5,          // four of a kind
        [5] => 6,             // five of a kind
        _ => panic!("Invalid hand combination!"),
    }
}

fn get_card_value(card: &char, joker: bool) -> u8 {
    match card {
        'T' => 10,
        'J' => {
            if joker {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).expect("Should be one of 2..=9") as u8,
    }
}

fn get_solution(input: &str, joker: bool) -> u32 {
    let mut hands = parse_input(input);
    hands.sort_unstable_by(|a, b| cmp_hands(a, b, joker));
    hands.iter().enumerate().fold(0, |acc, pair| {
        let (rank, hand) = pair;
        acc + (1 + rank as u32) * hand.bid
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn case1() {
        assert_eq!(get_solution(TEST_INPUT, false), 6440);
    }

    #[test]
    fn case2() {
        assert_eq!(get_solution(TEST_INPUT, true), 5905);
    }
}
