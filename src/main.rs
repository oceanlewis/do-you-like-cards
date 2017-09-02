extern crate rand;
extern crate num_cpus;

use rand::Rng;
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CardSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
    Dummy,
}

#[derive(Debug, Copy)]
struct Card {
    suit: CardSuit,
    value: u32,
}

impl Card {
    fn new(suit: CardSuit, value: u32) -> Card {
        Card {
            suit: suit,
            value: value,
        }
    }
}

impl Clone for Card {
    fn clone(&self) -> Card {
        Card {
            suit: self.suit,
            value: self.value,
        }
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut new_deck = Deck { cards: Vec::new() };

        for i in 0..52 {
            let card_value = (i % 13) + 1;
            let suit_num = i / 13;

            let card_suit = match suit_num {
                0 => CardSuit::Spades,
                1 => CardSuit::Hearts,
                2 => CardSuit::Diamonds,
                3 => CardSuit::Clubs,
                _ => panic!("Invalid card value"),
            };
            new_deck.cards.push(Card::new(card_suit, card_value));
        }

        new_deck
    }

    fn shuffle(&mut self) {
        rand::thread_rng().shuffle(&mut self.cards);
    }

    fn split_into(&mut self, number: usize) -> Vec<Deck> {
        let splits: Vec<Deck> = self.cards
            .chunks(number)
            .map(|chunk| Deck { cards: chunk.to_vec() })
            .collect();
        splits
    }

    fn lowest_card(&self, suit: CardSuit) -> Option<Card> {
        self.cards
            .iter()
            .filter(|card| card.suit == suit)
            .min_by(|a, b| a.value.cmp(&b.value))
            .map(|card| card.clone())
    }
}

fn spades_computation(iterations: usize) -> f64 {
    let mut cards_for_status: Vec<Card> = Vec::with_capacity(iterations);

    for _ in 1..iterations {
        let mut deck = Deck::new();
        deck.shuffle();

        let hands = deck.split_into(13 as usize);
        let lowest_cards: Vec<Card> = hands
            .iter()
            .map(|hand| match hand.lowest_card(CardSuit::Spades) {
                Some(card) => card,
                None => Card {
                    suit: CardSuit::Dummy,
                    value: 0,
                },
            })
            .collect();

        let lowest_card = lowest_cards.iter().max_by(|a, b| a.value.cmp(&b.value));
        cards_for_status.push(lowest_card.unwrap().to_owned());
    }

    let total = cards_for_status.iter().fold(
        0,
        |acc, &card| acc + card.value,
    ) as f64;
    let num_cards = cards_for_status.len() as f64;
    let avg = total / num_cards;
    avg
}

fn main() {
    let iterations = 100_000;
    let num_cpus = num_cpus::get();
    let per_thread = iterations / num_cpus;

    let mut threads = Vec::with_capacity(num_cpus);
    for _ in 0..num_cpus {
        threads.push(thread::spawn(move || spades_computation(per_thread)));
    }

    let mut totals: f64 = 0.0;
    for thread in threads {
        match thread.join() {
            Ok(avg) => totals += avg,
            Err(err) => panic!(err),
        }
    }

    let average = totals / num_cpus as f64;

    println!("Average Card Won: {}", average);
}
