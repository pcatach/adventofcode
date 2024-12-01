use itertools::Itertools;
use std::{cmp::Ordering, fs};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    hand_type: HandType,
    bid: u64,
}

fn main() {
    let file_path = "input.txt";
    let hands_input = fs::read_to_string(file_path).expect("Could not open file");

    let mut hands = parse_hands(&hands_input);
    hands.sort_by(compare_hands);

    // let ordered_bids: Vec<u64> = hands.iter().map(|hand| hand.bid).collect();
    // println!("Ordered bids: {:?}", ordered_bids);

    let total_winnings: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid)
        .sum();

    println!("Total winnings: {total_winnings}");
}

fn parse_hands(hands_input: &str) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for hand in hands_input.lines() {
        let (cards, bid) = hand.split_once(' ').unwrap();

        let bid: u64 = bid.parse().expect("Bid must be a number");
        let cards: Vec<char> = cards.chars().collect();
        let cards: [char; 5] = cards.try_into().unwrap();

        let hand_type = get_hand_type(cards);

        hands.push(Hand {
            cards,
            hand_type,
            bid,
        })
    }
    hands
}

fn get_hand_type(cards: [char; 5]) -> HandType {
    println!("{:?}", cards);
    let mut card_counts: Vec<(usize, char)> = cards
        .into_iter()
        .sorted()
        .dedup_with_count()
        .sorted()
        .rev()
        .collect();

    jokerify(&mut card_counts);

    if card_counts.len() == 1 {
        HandType::FiveOfAKind
    } else if card_counts.len() == 5 {
        HandType::HighCard
    } else if card_counts.len() == 4 {
        HandType::OnePair
    } else if card_counts.len() == 3 {
        if card_counts[0].0 == 3 {
            HandType::ThreeOfAKind
        } else {
            HandType::TwoPair
        }
    } else if card_counts.len() == 2 {
        if card_counts[0].0 == 4 {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        }
    } else {
        panic!("Unexpected hand {:?}", cards);
    }
}

fn jokerify(card_counts: &mut Vec<(usize, char)>) {
    let joker_index = card_counts.iter().position(|x| x.1 == 'J');
    if joker_index == None {
        return;
    };
    let joker_index = joker_index.unwrap();
    let joker_count = card_counts[joker_index].0;

    if card_counts.len() > 1 {
        card_counts.remove(joker_index);
        card_counts[0].0 += joker_count;
    }
}

fn compare_hands(hand_1: &Hand, hand_2: &Hand) -> Ordering {
    match hand_1.hand_type.cmp(&hand_2.hand_type) {
        Ordering::Equal => compare_cards(hand_1, hand_2),
        v => v,
    }
}

fn compare_cards(hand_1: &Hand, hand_2: &Hand) -> Ordering {
    for (i, card_1) in hand_1.cards.iter().enumerate() {
        let result = compare_nth_card(card_1, &hand_2.cards[i]);
        // println!("{card_1} {:?} {}", result, hand_2.cards[i]);
        if result != Ordering::Equal {
            return result;
        }
    }
    panic!("Unexpected hands to compare: {:?} and {:?}", hand_1, hand_2)
}

fn compare_nth_card(card_1: &char, card_2: &char) -> Ordering {
    parse_card(card_1).cmp(&parse_card(card_2))
}

fn parse_card(card: &char) -> i8 {
    match card.to_string().parse() {
        Ok(value) => value,
        Err(_) => match card {
            'T' => 10,
            // 'J' => 11,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unexpected card {card}"),
        },
    }
}
