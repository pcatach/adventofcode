use std::fs;
use std::io;

// Card is defined by index, winning numbers, numbers you have and copies
type Card = (u32, Vec<u32>, Vec<u32>, usize);

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let cards_input = fs::read_to_string(file_path)?;

    // Computing points the wrong way
    let cards = parse_cards(&cards_input);
    let total_points: i32 = cards.iter().map(|card| compute_points(card)).sum();
    println!("Total number of points: {}", total_points);

    // Computing total number of scratchcards to right way
    let mut cards_won = cards.clone();
    for i in 0..cards_won.len() {
        let wins = compute_wins(&cards_won[i]);
        for j in (i + 1)..(i + 1 + wins as usize) {
            cards_won[j].3 += cards_won[i].3;
        }
    }
    // for card in cards_won {
    //     println!("{:?}", card);
    // }

    println!(
        "Total number of scratchcards: {}",
        cards_won.iter().map(|card| card.3).sum::<usize>()
    );
    Ok(())
}

fn parse_cards(cards_input: &str) -> Vec<Card> {
    return cards_input
        .lines()
        .map(|card_line| {
            let mut split = card_line.split([':', '|']);
            // println!("{}", card_line);
            let card_index = split
                .next()
                .expect("Failed to parse card index")
                .strip_prefix("Card")
                .expect("Couldn't find 'Card' prefix")
                .trim_start()
                .parse::<u32>()
                .expect("Failed to convert card number to u32");

            let winning_numbers = split
                .next()
                .expect("Failed to parse winning numbers")
                .split_whitespace()
                .map(|x| x.parse::<u32>().expect("Failed to parse winning number"))
                .collect();

            let numbers_you_have = split
                .next()
                .expect("Failed to parse numbers you have")
                .split_whitespace()
                .map(|x| x.parse::<u32>().expect("Failed to parse number you have"))
                .collect();

            return (card_index, winning_numbers, numbers_you_have, 1);
        })
        .collect();
}

fn compute_points(card: &Card) -> i32 {
    let wins = compute_wins(card);

    // println!("Card {:?}", card);
    // println!("won {}", wins);

    if wins > 0 {
        return 2_i32.pow((wins - 1) as u32);
    }
    return 0;
}

fn compute_wins(card: &Card) -> i32 {
    return card
        .2
        .iter()
        .filter(|number_you_have| card.1.contains(number_you_have))
        .count() as i32;
}
