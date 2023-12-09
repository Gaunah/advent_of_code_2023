use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer part1: {}", part1(input));
    // println!("Answer part2: {}", part2(input));
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

fn cmp_hands(a: &Hand, b: &Hand) -> Ordering {
    match get_hand_type(a).cmp(&get_hand_type(b)) {
        Ordering::Equal => a
            .cards
            .chars()
            .zip(b.cards.chars())
            .find_map(|(first_card, second_card)| {
                let first_val = get_card_value(&first_card);
                let second_val = get_card_value(&second_card);
                (first_val != second_val).then(|| first_val.cmp(&second_val))
            })
            .unwrap_or(Ordering::Equal),
        other => other,
    }
}

fn get_hand_type(hand: &Hand) -> u8 {
    let mut card_counts = HashMap::new();
    card_counts.reserve(5);
    for ch in hand.cards.chars() {
        *card_counts.entry(ch).or_insert(0) += 1;
    }
    card_counts.shrink_to_fit();

    let mut values: Vec<_> = card_counts.values().into_iter().collect();
    values.sort_unstable();
    match values.as_slice() {
        [1, 1, 1, 1, 1] => 0, // high card
        [1, 1, 1, 2] => 1,    // one pair
        [1, 2, 2] => 2,       // two pair
        [1, 1, 3] => 3,       // three of a kind
        [2, 3] => 4,          // full house
        [1, 4] => 5,          // four of a kind
        [5] => 6,             // five of a kind
        _ => panic!("There should not be any other combination!"),
    }
}

fn get_card_value(card: &char) -> u8 {
    match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).expect("Should be one of 2..=9") as u8,
    }
}

fn part1(input: &str) -> u32 {
    let mut hands = parse_input(input);
    hands.sort_unstable_by(|a, b| cmp_hands(a, b));
    hands.iter().enumerate().fold(0, |acc, pair| {
        let (rank, hand) = pair;
        acc + (1 + rank as u32) * hand.bid
    })
}

// fn part2(input: &str) -> u32 {
//     0
// }

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
        assert_eq!(part1(TEST_INPUT), 6440);
    }

    // #[test]
    // fn case2() {
    //     assert_eq!(part2(TEST_INPUT), 0);
    // }
}
