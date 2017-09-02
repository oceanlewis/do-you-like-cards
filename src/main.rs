extern crate num_cpus;

mod lib;

use lib::deck::*;
use lib::deck::card::*;
use std::thread;

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
